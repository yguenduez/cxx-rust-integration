use crate::foo;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        pub fn foo() -> ();
    }
}
