fn main() {
    tonic_build::compile_protos("proto/route_guide.proto")
        .unwrap_or_else(|e| panic!("Failed to compiler protos {:?}", e));
} 