use rust_lib::my_super_call;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn rust_from_cpp() -> Vec<String>;
    }
}

pub fn rust_from_cpp() -> Vec<String> {
    my_super_call()
}
