fn main() {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src")
        .compile(
            &["proto/v1beta1.proto", "proto/v1alpha.proto"],
            &["proto"],
        ).unwrap();
}
