//! `k8s-deviceplugin` provides API bindings for Kubernetes device plugins.
//!
//! # Example
//!
//! ```rust,no_run
//! use k8s_deviceplugin::v1beta1::registration_client::RegistrationClient;
//! use k8s_deviceplugin::v1beta1;
//! use tokio::net::UnixStream;
//! use std::convert::TryFrom;
//! use tonic::transport::{Endpoint, Uri};
//! use tower::service_fn;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let channel = Endpoint::try_from("http://[::]:50051")?
//!         .connect_with_connector(service_fn(|_: Uri| {
//!             UnixStream::connect(v1beta1::KUBELET_SOCKET)
//!         }))
//!         .await?;
//!     let mut client = RegistrationClient::new(channel);
//!     let request = tonic::Request::new(v1beta1::RegisterRequest {
//!         endpoint: format!("{}/fpgk8s.sock", v1beta1::KUBELET_SOCKET),
//!         resource_name: "fpgk8s.io/fpga".into(),
//!         version: v1beta1::VERSION.into(),
//!         options: None,
//!     });
//!
//!     let response = client.register(request).await?;
//!
//!     println!("RESPONSE={:?}", response);
//!     Ok(())
//! }
//! ```
//!

pub mod v1beta1 {
    tonic::include_proto!("v1beta1");

    /// Means that the device is healthy.
    pub const HEALTHY: &str = "Healthy";

    /// Means that the device is unhealthy.
    pub const UNHEALTHY: &str = "Unhealthy";

    /// Means current version of the API supported by kubelet.
    pub const VERSION: &str = "v1beta1";

    /// The folder the Device Plugin is expecting sockets to be on. Only
    /// privileged pods have access to this path. Note: Placeholder until we
    /// find a "standard path".
    pub const DEVICE_PLUGIN_PATH: &str = "/var/lib/kubelet/device-plugins/";

    /// The path of the kubelet registry socket.
    pub const KUBELET_SOCKET: &str = "/var/lib/kubelet/device-plugins/kubelet.sock";

    /// Avoid failed to run kubelet: bad socketPath, must be an absolute path:
    /// /var/lib/kubelet/device-plugins/kubelet.sock
    /// https://github.com/kubernetes/kubernetes/issues/93262
    /// https://github.com/kubernetes/kubernetes/pull/93285#discussion_r458140701
    pub const DEVICE_PLUGIN_WINDOWS: &str = "\\var\\lib\\kubelet\\device-plugins\\";

    /// The path of the Kubelet registry socket on windows.T
    pub const KUBELET_SOCKET_WINDOWS: &str = "\\var\\lib\\kubelet\\device-plugins\\kubelet.sock";

    /// Timeout duration in secs for PreStartContainer RPC.
    pub const KUBELET_PRE_START_CONTAINER_RPC_TIMEOUT_IN_SECS: u64 = 30;
}

pub mod v1alpha {
    tonic::include_proto!("deviceplugin");

    /// Means that the device is healthy.
    pub const HEALTHY: &str = "Healthy";

    /// Means that the device is unhealthy.
    pub const UNHEALTHY: &str = "Unhealthy";

    /// The folder the Device Plugin is expecting sockets to be on. Only
    /// privileged pods have access to this path. Note: Placeholder until we
    /// find a "standard path".
    pub const DEVICE_PLUGIN_PATH: &str = "/var/lib/kubelet/device-plugins/";

    /// The path of the Kubelet registry socket.
    pub const KUBELET_SOCKET: &str = "/var/lib/kubelet/device-plugins/kubelet.sock";
}
