fn main() {
    println!("cargo:rerun-if-changed=tests/build.rs");
    println!("cargo:rerun-if-changed=tests/json-schema-test-suite");
}
