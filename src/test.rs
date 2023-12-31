use crate::db;
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref INITIATED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

#[cfg(test)]
pub fn init() {
    let mut initiated = INITIATED.lock().unwrap();
    if *initiated == false {
        dotenv().ok();
        db::init();
        *initiated = true;
    }
}
