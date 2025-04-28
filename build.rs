fn main() {
    let metadata = cargo_metadata::MetadataCommand::new().exec().unwrap();

    let version = metadata
        .root_package()
        .unwrap()
        .dependencies
        .iter()
        .find(|e| &e.name == "tonic-build")
        .unwrap()
        .req
        .to_string();

    println!("cargo:rustc-env=TONIC_BUILD_VERSION={version}");
}
