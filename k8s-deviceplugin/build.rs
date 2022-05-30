fn main() {
    tonic_build::configure()
        .build_server(true)
        .compile(&["proto/v1beta1.proto", "proto/v1alpha.proto"], &["proto"])
        .unwrap();
}
