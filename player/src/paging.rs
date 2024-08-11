use atomic_shim::AtomicU64;
use blaseball_vcr::db_manager::DatabaseManager;
use blaseball_vcr::etypes::DynamicEntity;
use blaseball_vcr::{ChroniclerEntity, VCRResult};
use moka::sync::Cache;
use parking_lot::Mutex;
use std::sync::{atomic, Arc};
use std::time::Duration;

pub type DynamicChronEntity = ChroniclerEntity<DynamicEntity>;

// a page manager. page ids are simple, sequential integers, encoded in hex form
pub struct PageManager {
    page_table: Cache<u64, Arc<Mutex<Page>>>,
    id_counter: AtomicU64,
}

impl PageManager {
    pub fn new(capacity: u64, time_to_idle: Duration) -> PageManager {
        PageManager {
            id_counter: AtomicU64::new(0),
            page_table: Cache::builder()
                .max_capacity(capacity)
                .time_to_idle(time_to_idle)
                .build(),
        }
    }

    /// adds a page, returning it's id
    #[inline(always)]
    pub fn add_page(&self, page: Page) -> u64 {
        let id = self.next_id();
        self.page_table.insert(id, Arc::new(Mutex::new(page)));
        id
    }

    /// gets a page by it's id
    #[inline(always)]
    pub fn get_page(&self, id: &u64) -> Option<Arc<Mutex<Page>>> {
        self.page_table.get(id)
    }

    #[inline(always)]
    fn next_id(&self) -> u64 {
        self.id_counter.fetch_add(1, atomic::Ordering::Relaxed)
    }
}

pub enum PageFetchParameters {
    Versions { before: i64, after: i64 },
    Entities { at: i64 },
}

pub struct Page {
    // ids still not fetched
    remaining_ids: Vec<[u8; 16]>,
    remaining_data: Vec<DynamicChronEntity>,
    parameters: PageFetchParameters,
}

impl Page {
    pub fn entities(at: i64, ids: Vec<[u8; 16]>) -> Page {
        Page {
            remaining_ids: ids,
            remaining_data: Vec::new(), // the vec will allocate more efficiently in fetch_next_entities
            parameters: PageFetchParameters::Entities { at },
        }
    }

    pub fn versions(before: i64, after: i64, ids: Vec<[u8; 16]>) -> Page {
        Page {
            remaining_ids: ids,
            remaining_data: Vec::new(),
            parameters: PageFetchParameters::Versions { before, after },
        }
    }

    /// can we get any more data out of this pager?
    pub fn is_empty(&self) -> bool {
        self.remaining_data.is_empty() && self.remaining_ids.is_empty()
    }

    pub fn take_n<T: 'static + Into<DynamicEntity>>(
        &mut self,
        db: &DatabaseManager,
        count: usize,
    ) -> VCRResult<Vec<DynamicChronEntity>> {
        let mut output: Vec<DynamicChronEntity> = Vec::with_capacity(std::cmp::min(1000, count));
        output.append(&mut self.remaining_data);

        // if we have less entities than requested cached, get some more
        if output.len() < count && !self.remaining_ids.is_empty() {
            use PageFetchParameters::*;
            match self.parameters {
                Entities { at } => {
                    // how many more entities do we need to fulfill the requested amount?
                    self.fetch_next_entities::<T>(db, count - output.len(), at)?;
                    output.append(&mut self.remaining_data);
                }
                Versions { before, after } => {
                    self.fetch_next_versions::<T>(db, count, before, after)?;
                    let remainder = count.saturating_sub(output.len());

                    // if we have more data than fits in the requested count, use Vec::drain
                    if self.remaining_data.len() > remainder {
                        output.extend(self.remaining_data.drain(..remainder));
                    // else, use Vec::append
                    } else {
                        output.append(&mut self.remaining_data);
                    }
                }
            }
        }

        Ok(output)
    }

    #[inline]
    fn fetch_next_versions<T: 'static + Into<DynamicEntity>>(
        &mut self,
        db: &DatabaseManager,
        total_needed: usize,
        before: i64,
        after: i64,
    ) -> VCRResult<()> {
        // TODO: add a clause for pre-allocating space via getting the total number of versions for an entity id

        // while we're under our target for total amount of data, get the next id and fetch it's versions
        while self.remaining_data.len() < total_needed {
            if let Some(id) = self.remaining_ids.pop() {
                if let Some(data) = db.get_versions::<T>(&id, before, after)? {
                    self.remaining_data
                        .extend(data.into_iter().map(ChroniclerEntity::erase))
                }
            } else {
                break;
            }
        }

        Ok(())
    }

    // fetch a block of n entites
    #[inline]
    fn fetch_next_entities<T: 'static + Into<DynamicEntity>>(
        &mut self,
        db: &DatabaseManager,
        count: usize,
        at: i64,
    ) -> VCRResult<()> {
        let ids: Vec<[u8; 16]> = self
            .remaining_ids
            .drain(..std::cmp::min(self.remaining_ids.len(), count))
            .collect();

        // force the vec to fully allocate for the extra capacity needed, if it needs to do so
        // this (hopefully) avoids further allocations in self.remaining_data.extend
        if let Some(extra_capacity_needed) = self.remaining_data.capacity().checked_sub(count) {
            self.remaining_data.reserve(extra_capacity_needed);
        }

        let data: Vec<Option<ChroniclerEntity<T>>> = db.get_entities(&ids[..], at)?;
        self.remaining_data.extend(
            data.into_iter()
                .filter_map(|entity| entity.map(ChroniclerEntity::erase)),
        );

        Ok(())
    }
}
