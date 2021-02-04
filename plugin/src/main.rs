use k8s_deviceplugin::v1beta1::registration_client::RegistrationClient;
use k8s_deviceplugin::v1beta1;
use tokio::net::UnixStream;
use std::convert::TryFrom;
use tonic::transport::{Endpoint, Uri};
use tower::service_fn;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Endpoint::try_from("http://[::]:50051")?
        .connect_with_connector(service_fn(|_: Uri| {
            UnixStream::connect(v1beta1::KUBELET_SOCKET)
        }))
        .await?;
    let mut client = RegistrationClient::new(channel);
    let request = tonic::Request::new(v1beta1::RegisterRequest {
        endpoint: format!("{}/fpgk8s.sock", v1beta1::KUBELET_SOCKET),
        resource_name: "fpgk8s.io/fpga".into(),
        version: v1beta1::VERSION.into(),
        options: None,
    });

    let response = client.register(request).await?;

    println!("RESPONSE={:?}", response);
    Ok(())
}