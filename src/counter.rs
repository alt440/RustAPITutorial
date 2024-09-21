use lazy_static::lazy_static;

use std::sync::Mutex;

lazy_static! {
    // better to use RwLock for complex objects
    static ref GLOBAL_COUNTER: Mutex<i32> = Mutex::new(100);
}

pub fn increment_counter() {
    let mut num = GLOBAL_COUNTER.lock().unwrap();
    *num += 1;
}

pub fn decrement_counter() {
    let mut num = GLOBAL_COUNTER.lock().unwrap();
    *num -= 1;
}

pub fn get_counter() -> i32 {
    let num = GLOBAL_COUNTER.lock().unwrap();
    *num
}