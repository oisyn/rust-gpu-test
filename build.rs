use spirv_builder::{MetadataPrintout, SpirvBuilder};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    SpirvBuilder::new("shaders", "spirv-unknown-vulkan1.1")
    .print_metadata(MetadataPrintout::DependencyOnly)
    .multimodule(true)
    .build()
    .unwrap();
    println!("Done!");
}

