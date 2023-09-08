fn main() {
    cc::Build::new()
        .file("src/idk_yet/interrupts/wrapper.c")
        .compile("wrapper");

    println!("cargo:rerun-if-changed=src/idk_yet/interrupts/wrapper.c");
}