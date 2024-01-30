fn main() {
    cxx_build::bridge("src/cxx_api.rs")
        .flag_if_supported("-std=c++17")
        .compile("rusttocpp")
}
