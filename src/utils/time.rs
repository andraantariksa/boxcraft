use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_timestamp() -> f32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f32()
}
