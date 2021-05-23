use std::any::type_name;
use std::time::SystemTime;


pub fn get_struct_name<T>() -> String {
    let v: Vec<&str> = type_name::<T>().split("::").collect();
    v.last().unwrap().to_lowercase()
}

pub fn get_timestamp_now() -> u64 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}
