use super::*;
use crate::*;
use rocket::serde::json::Json as RocketJSON;

use rocket::State;

use blaseball_vcr::db_manager::DatabaseManager;

use blaseball_vcr::stream_data::thisidisstaticyo;
use blaseball_vcr::*;

use vcr_schemas::*;

#[get("/v2/entities?<req..>")]
pub fn entities(
    req: EntitiesRequest<'_>,
    db_manager: &State<Arc<DatabaseManager>>,
    page_manager: &State<PageManager>,
) -> VCRResult<RocketJSON<DynChronResponse>> {
    let ety = req.ty.to_lowercase();

    if let Some(page_token) = req
        .page
        .as_ref()
        .and_then(|v| u64::from_str_radix(v, 16).ok())
    {
        let page_mutex = page_manager
            .get_page(&page_token)
            .ok_or(VCRError::InvalidPageToken)?;
        let mut page = page_mutex.lock();
        let data = call_method_by_type_with_custom_impls!(
            page.take_n,
            (db_manager, req.count.unwrap_or(100)),
            ety.as_str(),
            { return Err(VCRError::EntityTypeNotFound) }
        )?;

        Ok(RocketJSON(ChronResponse {
            next_page: if page.is_empty() {
                None
            } else {
                Some(req.page.unwrap())
            },
            data,
        }))
    } else {
        let at = req.at_nanos().unwrap_or(i64::MAX);

        let ids = if let Some(id) = req.id {
            vec![*id.as_bytes()]
        } else {
            call_method_by_type_with_custom_impls!(db_manager.all_entity_ids, (), ety.as_str(), {
                return Err(VCRError::EntityTypeNotFound);
            })
            .ok_or(VCRError::EntityTypeNotFound)?
            .to_vec()
        };

        let mut page = Page::entities(at, ids);
        let data = call_method_by_type_with_custom_impls!(
            page.take_n,
            (db_manager, req.count.unwrap_or(100)),
            ety.as_str(),
            { return Err(VCRError::EntityTypeNotFound) }
        )?;

        // if the page isn't empty, add it to the manager
        let token = if !page.is_empty() {
            Some(page_manager.add_page(page))
        } else {
            None
        };

        Ok(RocketJSON(ChronResponse {
            next_page: token.map(|v| format!("{:X}", v)),
            data,
        }))
    }
}

#[get("/v2/versions?<req..>")]
pub fn versions(
    req: VersionsRequest<'_>,
    db_manager: &State<Arc<DatabaseManager>>,
    page_manager: &State<PageManager>,
) -> VCRResult<RocketJSON<DynChronResponse>> {
    let ety = req.ty.to_lowercase();

    if let Some(page_token) = req
        .page
        .as_ref()
        .and_then(|v| u64::from_str_radix(v, 16).ok())
    {
        let page_mutex = page_manager
            .get_page(&page_token)
            .ok_or(VCRError::InvalidPageToken)?;
        let mut page = page_mutex.lock();
        let data = call_method_by_type_with_custom_impls!(
            page.take_n,
            (db_manager, req.count.unwrap_or(100)),
            ety.as_str(),
            { return Err(VCRError::EntityTypeNotFound) }
        )?;

        Ok(RocketJSON(ChronResponse {
            next_page: if page.is_empty() {
                None
            } else {
                Some(req.page.unwrap())
            },
            data,
        }))
    } else {
        let step = 3;

        // if ety == "stream" {
        //     let start_time = req.after_nanos().unwrap_or_else(|| {
        //         req.before_nanos().unwrap_or(i64::MAX) - ((req.count.unwrap_or(1) as i64) * step)
        //     });
        //     // req.before_nanos() - ((req.count.unwrap_or(1) as i64) * step),

        //     let end_time = req.before_nanos().unwrap_or_else(|| {
        //         req.after_nanos().unwrap_or(0) + ((req.count.unwrap_or(1) as i64) * step)
        //     });

        //     let mut stream_samples = (start_time..end_time)
        //         .step_by(step as usize)
        //         .map(|at| {
        //             Ok((ChroniclerEntity {
        //                 entity_id: *uuid::Uuid::nil().as_bytes(),
        //                 valid_from: at,
        //                 data: blaseball_vcr::stream_data::stream_data(db_manager, at)?,
        //             })
        //             .erase())
        //         })
        //         .collect::<VCRResult<Vec<DynamicChronEntity>>>()?;

        //     stream_samples.sort_by_key(|x| x.valid_from);
        //     if let Some(Order::Desc) = req.order {
        //         stream_samples.reverse();
        //     }

        //     return Ok(RocketJSON(ChronResponse {
        //         next_page: None,
        //         data: stream_samples,
        //     }));
        // }

        let before = req.before_nanos().unwrap_or(i64::MAX);
        let after = req.after_nanos().unwrap_or(i64::MAX);

        let ids = if let Some(id) = req.id {
            vec![*id.as_bytes()]
        } else {
            call_method_by_type_with_custom_impls!(db_manager.all_entity_ids, (), ety.as_str(), {
                return Err(VCRError::EntityTypeNotFound);
            })
            .ok_or(VCRError::EntityTypeNotFound)?
            .to_vec()
        };

        let mut page = Page::versions(before, after, ids);
        let data = call_method_by_type_with_custom_impls!(
            page.take_n,
            (db_manager, req.count.unwrap_or(100)),
            ety.as_str(),
            { return Err(VCRError::EntityTypeNotFound) }
        )?;

        // if the page isn't empty, add it to the manager
        let token = if !page.is_empty() {
            Some(page_manager.add_page(page))
        } else {
            None
        };

        Ok(RocketJSON(ChronResponse {
            next_page: token.map(|v| format!("{:X}", v)),
            data,
        }))
    }
}
