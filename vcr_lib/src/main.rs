use blaseball_vcr::vhs::schemas::*;
use blaseball_vcr::{vhs::db, EntityDatabase, VCRResult};

use uuid::Uuid;

fn main() -> VCRResult<()> {
    let db: db::Database<IdolsWrapper> = db::Database::from_single("./vhs_tapes/idols.vhs")?;
    db.get_entity(&Uuid::nil().as_bytes(), 1600719885);
    // 0 - 3
    // println!("{}", serde_json::to_string_pretty(&db.get_versions_inner(&Uuid::nil().as_bytes(), 1596294001, 1596266001)?).unwrap());

    // 98 - 100
    // println!("{}", serde_json::to_string_pretty(&db.get_versions_inner(&Uuid::nil().as_bytes(), 1596650400, 1596646801)?).unwrap());

    // 0 - 272
    // let after = 1596266001;
    // let before = 1598443202;
    // let v = db
    //     .get_versions(&Uuid::nil().as_bytes(), before, after)?
    //     .unwrap();

    // println!("{}", v.len());
    // println!("{}", serde_json::to_string_pretty(&v.last())?);

    Ok(())
}
