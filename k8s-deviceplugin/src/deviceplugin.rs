#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterRequest {
    /// Version of the API the Device Plugin was built against
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    /// Name of the unix socket the device plugin is listening on
    /// PATH = path.Join(DevicePluginPath, endpoint)
    #[prost(string, tag = "2")]
    pub endpoint: ::prost::alloc::string::String,
    /// Schedulable resource name. As of now it's expected to be a DNS Label
    #[prost(string, tag = "3")]
    pub resource_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
/// ListAndWatch returns a stream of List of Devices
/// Whenever a Device state changes or a Device disappears, ListAndWatch
/// returns the new list
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAndWatchResponse {
    #[prost(message, repeated, tag = "1")]
    pub devices: ::prost::alloc::vec::Vec<Device>,
}
/// E.g:
/// struct Device {
///    ID: "GPU-fef8089b-4820-abfc-e83e-94318197576e",
///    Health: "Healthy",
///}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Device {
    /// A unique ID assigned by the device plugin used
    /// to identify devices during the communication
    /// Max length of this field is 63 characters
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Health of the device, can be healthy or unhealthy, see constants.go
    #[prost(string, tag = "2")]
    pub health: ::prost::alloc::string::String,
}
/// - Allocate is expected to be called during pod creation since allocation
///   failures for any container would result in pod startup failure.
/// - Allocate allows kubelet to exposes additional artifacts in a pod's
///   environment as directed by the plugin.
/// - Allocate allows Device Plugin to run device specific operations on
///   the Devices requested
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocateRequest {
    #[prost(string, repeated, tag = "1")]
    pub devices_i_ds: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// AllocateResponse includes the artifacts that needs to be injected into
/// a container for accessing 'deviceIDs' that were mentioned as part of
/// 'AllocateRequest'.
/// Failure Handling:
/// if Kubelet sends an allocation request for dev1 and dev2.
/// Allocation on dev1 succeeds but allocation on dev2 fails.
/// The Device plugin should send a ListAndWatch update and fail the
/// Allocation request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocateResponse {
    /// List of environment variable to be set in the container to access one of more devices.
    #[prost(map = "string, string", tag = "1")]
    pub envs:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Mounts for the container.
    #[prost(message, repeated, tag = "2")]
    pub mounts: ::prost::alloc::vec::Vec<Mount>,
    /// Devices for the container.
    #[prost(message, repeated, tag = "3")]
    pub devices: ::prost::alloc::vec::Vec<DeviceSpec>,
    /// Container annotations to pass to the container runtime
    #[prost(map = "string, string", tag = "4")]
    pub annotations:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Mount specifies a host volume to mount into a container.
/// where device library or tools are installed on host and container
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mount {
    /// Path of the mount within the container.
    #[prost(string, tag = "1")]
    pub container_path: ::prost::alloc::string::String,
    /// Path of the mount on the host.
    #[prost(string, tag = "2")]
    pub host_path: ::prost::alloc::string::String,
    /// If set, the mount is read-only.
    #[prost(bool, tag = "3")]
    pub read_only: bool,
}
/// DeviceSpec specifies a host device to mount into a container.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceSpec {
    /// Path of the device within the container.
    #[prost(string, tag = "1")]
    pub container_path: ::prost::alloc::string::String,
    /// Path of the device on the host.
    #[prost(string, tag = "2")]
    pub host_path: ::prost::alloc::string::String,
    /// Cgroups permissions of the device, candidates are one or more of
    /// * r - allows container to read from the specified device.
    /// * w - allows container to write to the specified device.
    /// * m - allows container to create device files that do not yet exist.
    #[prost(string, tag = "3")]
    pub permissions: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod registration_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Registration is the service advertised by the Kubelet"]
    #[doc = " Only when Kubelet answers with a success code to a Register Request"]
    #[doc = " may Device Plugins start their service"]
    #[doc = " Registration may fail when device plugin version is not supported by"]
    #[doc = " Kubelet or the registered resourceName is already taken by another"]
    #[doc = " active device plugin. Device plugin is expected to terminate upon registration failure"]
    pub struct RegistrationClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RegistrationClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> RegistrationClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn register(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterRequest>,
        ) -> Result<tonic::Response<super::Empty>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/deviceplugin.Registration/Register");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for RegistrationClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for RegistrationClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "RegistrationClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod device_plugin_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " DevicePlugin is the service advertised by Device Plugins"]
    pub struct DevicePluginClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DevicePluginClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DevicePluginClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " ListAndWatch returns a stream of List of Devices"]
        #[doc = " Whenever a Device state changes or a Device disappears, ListAndWatch"]
        #[doc = " returns the new list"]
        pub async fn list_and_watch(
            &mut self,
            request: impl tonic::IntoRequest<super::Empty>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ListAndWatchResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/deviceplugin.DevicePlugin/ListAndWatch");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Allocate is called during container creation so that the Device"]
        #[doc = " Plugin can run device specific operations and instruct Kubelet"]
        #[doc = " of the steps to make the Device available in the container"]
        pub async fn allocate(
            &mut self,
            request: impl tonic::IntoRequest<super::AllocateRequest>,
        ) -> Result<tonic::Response<super::AllocateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/deviceplugin.DevicePlugin/Allocate");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for DevicePluginClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DevicePluginClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DevicePluginClient {{ ... }}")
        }
    }
}
