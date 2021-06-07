use std::any::type_name;


pub fn get_struct_name<T>() -> String {
    let v: Vec<&str> = type_name::<T>().split("::").collect();
    v.last().unwrap().to_lowercase()
}
