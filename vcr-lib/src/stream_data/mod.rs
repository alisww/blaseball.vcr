use bitcode::{Decode, Encode};

use crate::{db_manager::DatabaseManager, EntityLocation, VCRResult};

pub mod db;
pub mod thisidisstaticyo;
// pub struct EntityIndexAndTime {

// }

#[derive(Encode, Decode)]
pub struct StreamDataBatch<I> {
    pub times: Vec<i64>,
    pub items: Vec<I>,
}

pub trait StreamComponent {
    type Packed: PackedStreamComponent;

    fn pack(
        &self,
        time: i64,
        location_table: &redb::ReadOnlyTable<u128, EntityLocation>,
        database: &DatabaseManager,
    ) -> VCRResult<Self::Packed>;
}

pub trait PackedStreamComponent: for<'de> bitcode::Decode<'de> + bitcode::Encode {
    type Unpacked: StreamComponent;

    fn unpack(&self, time: i64, database: &DatabaseManager) -> VCRResult<Self::Unpacked>;
}

#[macro_export]
macro_rules! pack_entities {
    (list of $etype:ty, $source:expr, $table:expr) => {
        if let Some(items) = $source {
            let mut serialize_buffer = Vec::new();
            let mut locations: Vec<EntityLocation> = Vec::with_capacity(items.len());

            for item in items {
                <$etype as BorshSerialize>::serialize(item, &mut serialize_buffer).unwrap();
                let hash = xxh3_128(&serialize_buffer);

                let location = $table
                    .get(hash)?
                    .expect("failed to match streamdata component");
                locations.push(location.value());

                serialize_buffer.clear();
            }

            Some(locations)
        } else {
            None
        }
    };
    (list of $etype:ty, fallback, $time:expr, $source:expr, $db:expr, $table:expr, $extractor:expr) => {
        if let Some(items) = $source {
            let mut serialize_buffer = Vec::new();
            let mut locations: Vec<EntityLocation> = Vec::with_capacity(items.len());
            let db = $db.get_db::<$etype>().unwrap();

            for item in items {
                <$etype as BorshSerialize>::serialize(item, &mut serialize_buffer).unwrap();
                let hash = xxh3_128(&serialize_buffer);

                let location = if let Some(location) = $table.get(hash)? {
                    location.value()
                } else {
                    let id = $extractor(item);
                    let Some(idx) = db.index_from_id(id.as_bytes()) else {
                        println!(
                            "failed to find id {id} for {}",
                            std::any::type_name::<$etype>()
                        );
                        continue;
                    };

                    let header = db.header_by_index(idx).unwrap();
                    let time = header.find_time_unchecked($time);
                    EntityLocation {
                        header_index: idx as u32,
                        time_index: time as u32,
                    }
                };

                locations.push(location);

                serialize_buffer.clear();
            }

            Some(locations)
        } else {
            None
        }
    };
    (by id list of $etype:ty, $time: expr, $source:expr, $db:expr, $extractor:expr) => {
        if let Some(items) = $source {
            let mut positions = Vec::with_capacity(items.len());
            let db = $db.get_db::<$etype>().unwrap();

            for item in items {
                let id = $extractor(item);
                let Some(idx) = db.index_from_id(id.as_bytes()) else {
                    println!(
                        "failed to find id {id} for {}",
                        std::any::type_name::<$etype>()
                    );
                    continue;
                };

                let header = db.header_by_index(idx).unwrap();
                let time = header.find_time_unchecked($time);
                positions.push(EntityLocation {
                    header_index: idx as u32,
                    time_index: time as u32,
                });
            }

            Some(positions)
        } else {
            None
        }
    };
    (one of $etype:ty, fallback, $time:expr, $source:expr, $db:expr, $table:expr, $extractor:expr) => {
        if let Some(item) = $source {
            let mut serialize_buffer = Vec::new();

            <$etype as BorshSerialize>::serialize(item, &mut serialize_buffer).unwrap();
            let hash = xxh3_128(&serialize_buffer);

            if let Some(location) = $table.get(hash)? {
                Some(location.value())
            } else {
                let db = $db.get_db::<$etype>().unwrap();

                let id = $extractor(item);
                if let Some(idx) = db.index_from_id(id.as_bytes()) {
                    let header = db.header_by_index(idx).unwrap();
                    let time = header.find_time_unchecked($time);

                    // let db = $db.get_db::<$etype>().unwrap();
                    // let id = $extractor(item);
                    Some(EntityLocation {
                        header_index: idx,
                        time_index: time as u32,
                    })
                } else {
                    None
                }
            }
        } else {
            None
        }
    };
    (one of $etype:ty, $source:expr, $table:expr) => {
        if let Some(item) = $source {
            let mut serialize_buffer = Vec::new();

            <$etype as BorshSerialize>::serialize(item, &mut serialize_buffer).unwrap();
            let hash = xxh3_128(&serialize_buffer);

            let location = $table
                .get(hash)?
                .expect("failed to match streamdata component");

            Some(location.value())
        } else {
            None
        }
    };
    (by id one of $etype:ty, $time:expr, $source:expr, $db:expr, $extractor:expr) => {
        if let Some(item) = $source {
            let db = $db.get_db::<$etype>().unwrap();

            let id = $extractor(item);
            if let Some(idx) = db.index_from_id(id.as_bytes()) {
                let header = db.header_by_index(idx).unwrap();
                let time = header.find_time_unchecked($time);

                // let db = $db.get_db::<$etype>().unwrap();
                // let id = $extractor(item);
                Some(EntityLocation {
                    header_index: idx,
                    time_index: time as u32,
                })
            } else {
                None
            }
        } else {
            None
        }
    };
}

#[macro_export]
macro_rules! unpack_entities {
    (list of $etype:ty, $time:expr, $source:expr, $db:expr) => {
        if let Some(items) = $source {
            let db = $db.get_db::<$etype>().unwrap();
            let items = db.get_entities_by_location(&items, true)?.into_iter().map(|v| v.unwrap().data).collect::<Vec<_>>();
            Some(items)
        } else {
            None
        }
    };
    (one of $etype:ty, $time:expr, $source:expr, $db:expr) => {
        if let Some(item) = $source {
            let db = $db.get_db::<$etype>().unwrap();
            let item = db.get_entity_by_location(item)?.unwrap().data;
            // let items = db.get_entities_by_indices(&items, $time, false)?.into_iter().map(|v| v.unwrap().data).collect::<Vec<_>>();
            Some(item)
        } else {
            None
        }
    }
}
