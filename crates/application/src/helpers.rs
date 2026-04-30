use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
