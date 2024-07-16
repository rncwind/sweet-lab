fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .out_dir("src/")
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(&["proto/helloworld.proto"], &["proto/"])
        .unwrap();
    Ok(())
}
