#[cxx::bridge]
mod ffi {
    extern "Rust" {
        pub fn my_function_from_rust() -> String;
    }
}

pub fn my_function_from_rust() -> String {
    "Hello from Rust!".to_string()
}
