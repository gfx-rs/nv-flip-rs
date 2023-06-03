fn main() {
    cc::Build::new()
        .cpp(true)
        .files(["src/bindings.cpp"])
        .flag_if_supported("-std=c++17")
        .includes(["src/", "extern/cpp/common/", "extern/cpp/CPP/"])
        .compile("flip");

    println!("cargo:rerun-if-changed=src/bindings.cpp");
    println!("cargo:rerun-if-changed=src/bindings.hpp");
}
