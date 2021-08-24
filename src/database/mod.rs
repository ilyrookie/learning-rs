use diesel::prelude::*;
use diesel::pg::PgConnection;

static mut DB: Option<PgConnection> = None;

pub fn connect(db_url: &str) {
    unsafe{
    DB = Some(PgConnection::establish(db_url)
        .expect("Unable to connect to database".as_ref()))
    }
}   

pub fn get_connection<'a>() -> &'a Option<PgConnection> {
    unsafe{&DB}
}