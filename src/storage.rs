use gloo_storage::{LocalStorage, Storage};

pub fn get<T: for<'de> serde::Deserialize<'de>>(key: &str) -> gloo_storage::Result<T> {
    LocalStorage::get(key)
}

pub fn set<T: serde::ser::Serialize>(key: &str, value: T) -> gloo_storage::Result<()> {
    LocalStorage::set(key, value)
}
