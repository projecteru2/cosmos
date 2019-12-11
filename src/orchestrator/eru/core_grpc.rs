// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait CoreRPC {
    fn list_networks(&self, o: ::grpc::RequestOptions, p: super::core::ListNetworkOptions) -> ::grpc::SingleResponse<super::core::Networks>;

    fn list_pods(&self, o: ::grpc::RequestOptions, p: super::core::Empty) -> ::grpc::SingleResponse<super::core::Pods>;

    fn add_pod(&self, o: ::grpc::RequestOptions, p: super::core::AddPodOptions) -> ::grpc::SingleResponse<super::core::Pod>;

    fn remove_pod(&self, o: ::grpc::RequestOptions, p: super::core::RemovePodOptions) -> ::grpc::SingleResponse<super::core::Empty>;

    fn get_pod(&self, o: ::grpc::RequestOptions, p: super::core::GetPodOptions) -> ::grpc::SingleResponse<super::core::Pod>;

    fn get_pod_resource(&self, o: ::grpc::RequestOptions, p: super::core::GetPodOptions) -> ::grpc::SingleResponse<super::core::PodResource>;

    fn list_pod_nodes(&self, o: ::grpc::RequestOptions, p: super::core::ListNodesOptions) -> ::grpc::SingleResponse<super::core::Nodes>;

    fn add_node(&self, o: ::grpc::RequestOptions, p: super::core::AddNodeOptions) -> ::grpc::SingleResponse<super::core::Node>;

    fn remove_node(&self, o: ::grpc::RequestOptions, p: super::core::RemoveNodeOptions) -> ::grpc::SingleResponse<super::core::Empty>;

    fn set_node(&self, o: ::grpc::RequestOptions, p: super::core::SetNodeOptions) -> ::grpc::SingleResponse<super::core::Node>;

    fn get_node(&self, o: ::grpc::RequestOptions, p: super::core::GetNodeOptions) -> ::grpc::SingleResponse<super::core::Node>;

    fn get_node_resource(&self, o: ::grpc::RequestOptions, p: super::core::GetNodeOptions) -> ::grpc::SingleResponse<super::core::NodeResource>;

    fn get_container(&self, o: ::grpc::RequestOptions, p: super::core::ContainerID) -> ::grpc::SingleResponse<super::core::Container>;

    fn get_containers(&self, o: ::grpc::RequestOptions, p: super::core::ContainerIDs) -> ::grpc::SingleResponse<super::core::Containers>;

    fn list_containers(&self, o: ::grpc::RequestOptions, p: super::core::ListContainersOptions) -> ::grpc::StreamingResponse<super::core::Container>;

    fn list_node_containers(&self, o: ::grpc::RequestOptions, p: super::core::GetNodeOptions) -> ::grpc::SingleResponse<super::core::Containers>;

    fn get_containers_status(&self, o: ::grpc::RequestOptions, p: super::core::ContainerIDs) -> ::grpc::SingleResponse<super::core::ContainersStatus>;

    fn set_containers_status(&self, o: ::grpc::RequestOptions, p: super::core::SetContainersStatusOptions) -> ::grpc::SingleResponse<super::core::ContainersStatus>;

    fn container_status_stream(&self, o: ::grpc::RequestOptions, p: super::core::ContainerStatusStreamOptions) -> ::grpc::StreamingResponse<super::core::ContainerStatusStreamMessage>;

    fn copy(&self, o: ::grpc::RequestOptions, p: super::core::CopyOptions) -> ::grpc::StreamingResponse<super::core::CopyMessage>;

    fn send(&self, o: ::grpc::RequestOptions, p: super::core::SendOptions) -> ::grpc::StreamingResponse<super::core::SendMessage>;

    fn build_image(&self, o: ::grpc::RequestOptions, p: super::core::BuildImageOptions) -> ::grpc::StreamingResponse<super::core::BuildImageMessage>;

    fn cache_image(&self, o: ::grpc::RequestOptions, p: super::core::CacheImageOptions) -> ::grpc::StreamingResponse<super::core::CacheImageMessage>;

    fn remove_image(&self, o: ::grpc::RequestOptions, p: super::core::RemoveImageOptions) -> ::grpc::StreamingResponse<super::core::RemoveImageMessage>;

    fn create_container(&self, o: ::grpc::RequestOptions, p: super::core::DeployOptions) -> ::grpc::StreamingResponse<super::core::CreateContainerMessage>;

    fn replace_container(&self, o: ::grpc::RequestOptions, p: super::core::ReplaceOptions) -> ::grpc::StreamingResponse<super::core::ReplaceContainerMessage>;

    fn remove_container(&self, o: ::grpc::RequestOptions, p: super::core::RemoveContainerOptions) -> ::grpc::StreamingResponse<super::core::RemoveContainerMessage>;

    fn dissociate_container(&self, o: ::grpc::RequestOptions, p: super::core::DissociateContainerOptions) -> ::grpc::StreamingResponse<super::core::DissociateContainerMessage>;

    fn control_container(&self, o: ::grpc::RequestOptions, p: super::core::ControlContainerOptions) -> ::grpc::StreamingResponse<super::core::ControlContainerMessage>;

    fn realloc_resource(&self, o: ::grpc::RequestOptions, p: super::core::ReallocOptions) -> ::grpc::StreamingResponse<super::core::ReallocResourceMessage>;

    fn log_stream(&self, o: ::grpc::RequestOptions, p: super::core::ContainerID) -> ::grpc::StreamingResponse<super::core::LogStreamMessage>;

    fn run_and_wait(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::core::RunAndWaitOptions>) -> ::grpc::StreamingResponse<super::core::AttachContainerMessage>;

    fn execute_container(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::core::ExecuteContainerOptions>) -> ::grpc::StreamingResponse<super::core::AttachContainerMessage>;
}

// client

pub struct CoreRPCClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_ListNetworks: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::ListNetworkOptions, super::core::Networks>>,
    method_ListPods: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::Empty, super::core::Pods>>,
    method_AddPod: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::AddPodOptions, super::core::Pod>>,
    method_RemovePod: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::RemovePodOptions, super::core::Empty>>,
    method_GetPod: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::GetPodOptions, super::core::Pod>>,
    method_GetPodResource: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::GetPodOptions, super::core::PodResource>>,
    method_ListPodNodes: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::ListNodesOptions, super::core::Nodes>>,
    method_AddNode: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::AddNodeOptions, super::core::Node>>,
    method_RemoveNode: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::RemoveNodeOptions, super::core::Empty>>,
    method_SetNode: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::SetNodeOptions, super::core::Node>>,
    method_GetNode: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::GetNodeOptions, super::core::Node>>,
    method_GetNodeResource: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::GetNodeOptions, super::core::NodeResource>>,
    method_GetContainer: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::ContainerID, super::core::Container>>,
    method_GetContainers: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::ContainerIDs, super::core::Containers>>,
    method_ListContainers: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::ListContainersOptions, super::core::Container>>,
    method_ListNodeContainers: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::GetNodeOptions, super::core::Containers>>,
    method_GetContainersStatus: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::ContainerIDs, super::core::ContainersStatus>>,
    method_SetContainersStatus: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::SetContainersStatusOptions, super::core::ContainersStatus>>,
    method_ContainerStatusStream: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::ContainerStatusStreamOptions, super::core::ContainerStatusStreamMessage>>,
    method_Copy: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::CopyOptions, super::core::CopyMessage>>,
    method_Send: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::SendOptions, super::core::SendMessage>>,
    method_BuildImage: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::BuildImageOptions, super::core::BuildImageMessage>>,
    method_CacheImage: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::CacheImageOptions, super::core::CacheImageMessage>>,
    method_RemoveImage: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::RemoveImageOptions, super::core::RemoveImageMessage>>,
    method_CreateContainer: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::DeployOptions, super::core::CreateContainerMessage>>,
    method_ReplaceContainer: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::ReplaceOptions, super::core::ReplaceContainerMessage>>,
    method_RemoveContainer: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::RemoveContainerOptions, super::core::RemoveContainerMessage>>,
    method_DissociateContainer: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::DissociateContainerOptions, super::core::DissociateContainerMessage>>,
    method_ControlContainer: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::ControlContainerOptions, super::core::ControlContainerMessage>>,
    method_ReallocResource: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::ReallocOptions, super::core::ReallocResourceMessage>>,
    method_LogStream: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::ContainerID, super::core::LogStreamMessage>>,
    method_RunAndWait: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::RunAndWaitOptions, super::core::AttachContainerMessage>>,
    method_ExecuteContainer: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::core::ExecuteContainerOptions, super::core::AttachContainerMessage>>,
}

impl ::grpc::ClientStub for CoreRPCClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        CoreRPCClient {
            grpc_client: grpc_client,
            method_ListNetworks: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/ListNetworks".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListPods: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/ListPods".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_AddPod: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/AddPod".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_RemovePod: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/RemovePod".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetPod: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/GetPod".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetPodResource: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/GetPodResource".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListPodNodes: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/ListPodNodes".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_AddNode: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/AddNode".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_RemoveNode: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/RemoveNode".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SetNode: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/SetNode".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetNode: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/GetNode".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetNodeResource: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/GetNodeResource".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetContainer: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/GetContainer".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetContainers: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/GetContainers".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListContainers: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/ListContainers".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListNodeContainers: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/ListNodeContainers".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetContainersStatus: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/GetContainersStatus".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SetContainersStatus: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/SetContainersStatus".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ContainerStatusStream: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/ContainerStatusStream".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Copy: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/Copy".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Send: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/Send".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_BuildImage: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/BuildImage".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_CacheImage: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/CacheImage".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_RemoveImage: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/RemoveImage".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_CreateContainer: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/CreateContainer".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ReplaceContainer: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/ReplaceContainer".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_RemoveContainer: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/RemoveContainer".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_DissociateContainer: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/DissociateContainer".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ControlContainer: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/ControlContainer".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ReallocResource: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/ReallocResource".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_LogStream: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/LogStream".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_RunAndWait: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/RunAndWait".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Bidi,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ExecuteContainer: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/pb.CoreRPC/ExecuteContainer".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Bidi,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl CoreRPC for CoreRPCClient {
    fn list_networks(&self, o: ::grpc::RequestOptions, p: super::core::ListNetworkOptions) -> ::grpc::SingleResponse<super::core::Networks> {
        self.grpc_client.call_unary(o, p, self.method_ListNetworks.clone())
    }

    fn list_pods(&self, o: ::grpc::RequestOptions, p: super::core::Empty) -> ::grpc::SingleResponse<super::core::Pods> {
        self.grpc_client.call_unary(o, p, self.method_ListPods.clone())
    }

    fn add_pod(&self, o: ::grpc::RequestOptions, p: super::core::AddPodOptions) -> ::grpc::SingleResponse<super::core::Pod> {
        self.grpc_client.call_unary(o, p, self.method_AddPod.clone())
    }

    fn remove_pod(&self, o: ::grpc::RequestOptions, p: super::core::RemovePodOptions) -> ::grpc::SingleResponse<super::core::Empty> {
        self.grpc_client.call_unary(o, p, self.method_RemovePod.clone())
    }

    fn get_pod(&self, o: ::grpc::RequestOptions, p: super::core::GetPodOptions) -> ::grpc::SingleResponse<super::core::Pod> {
        self.grpc_client.call_unary(o, p, self.method_GetPod.clone())
    }

    fn get_pod_resource(&self, o: ::grpc::RequestOptions, p: super::core::GetPodOptions) -> ::grpc::SingleResponse<super::core::PodResource> {
        self.grpc_client.call_unary(o, p, self.method_GetPodResource.clone())
    }

    fn list_pod_nodes(&self, o: ::grpc::RequestOptions, p: super::core::ListNodesOptions) -> ::grpc::SingleResponse<super::core::Nodes> {
        self.grpc_client.call_unary(o, p, self.method_ListPodNodes.clone())
    }

    fn add_node(&self, o: ::grpc::RequestOptions, p: super::core::AddNodeOptions) -> ::grpc::SingleResponse<super::core::Node> {
        self.grpc_client.call_unary(o, p, self.method_AddNode.clone())
    }

    fn remove_node(&self, o: ::grpc::RequestOptions, p: super::core::RemoveNodeOptions) -> ::grpc::SingleResponse<super::core::Empty> {
        self.grpc_client.call_unary(o, p, self.method_RemoveNode.clone())
    }

    fn set_node(&self, o: ::grpc::RequestOptions, p: super::core::SetNodeOptions) -> ::grpc::SingleResponse<super::core::Node> {
        self.grpc_client.call_unary(o, p, self.method_SetNode.clone())
    }

    fn get_node(&self, o: ::grpc::RequestOptions, p: super::core::GetNodeOptions) -> ::grpc::SingleResponse<super::core::Node> {
        self.grpc_client.call_unary(o, p, self.method_GetNode.clone())
    }

    fn get_node_resource(&self, o: ::grpc::RequestOptions, p: super::core::GetNodeOptions) -> ::grpc::SingleResponse<super::core::NodeResource> {
        self.grpc_client.call_unary(o, p, self.method_GetNodeResource.clone())
    }

    fn get_container(&self, o: ::grpc::RequestOptions, p: super::core::ContainerID) -> ::grpc::SingleResponse<super::core::Container> {
        self.grpc_client.call_unary(o, p, self.method_GetContainer.clone())
    }

    fn get_containers(&self, o: ::grpc::RequestOptions, p: super::core::ContainerIDs) -> ::grpc::SingleResponse<super::core::Containers> {
        self.grpc_client.call_unary(o, p, self.method_GetContainers.clone())
    }

    fn list_containers(&self, o: ::grpc::RequestOptions, p: super::core::ListContainersOptions) -> ::grpc::StreamingResponse<super::core::Container> {
        self.grpc_client.call_server_streaming(o, p, self.method_ListContainers.clone())
    }

    fn list_node_containers(&self, o: ::grpc::RequestOptions, p: super::core::GetNodeOptions) -> ::grpc::SingleResponse<super::core::Containers> {
        self.grpc_client.call_unary(o, p, self.method_ListNodeContainers.clone())
    }

    fn get_containers_status(&self, o: ::grpc::RequestOptions, p: super::core::ContainerIDs) -> ::grpc::SingleResponse<super::core::ContainersStatus> {
        self.grpc_client.call_unary(o, p, self.method_GetContainersStatus.clone())
    }

    fn set_containers_status(&self, o: ::grpc::RequestOptions, p: super::core::SetContainersStatusOptions) -> ::grpc::SingleResponse<super::core::ContainersStatus> {
        self.grpc_client.call_unary(o, p, self.method_SetContainersStatus.clone())
    }

    fn container_status_stream(&self, o: ::grpc::RequestOptions, p: super::core::ContainerStatusStreamOptions) -> ::grpc::StreamingResponse<super::core::ContainerStatusStreamMessage> {
        self.grpc_client.call_server_streaming(o, p, self.method_ContainerStatusStream.clone())
    }

    fn copy(&self, o: ::grpc::RequestOptions, p: super::core::CopyOptions) -> ::grpc::StreamingResponse<super::core::CopyMessage> {
        self.grpc_client.call_server_streaming(o, p, self.method_Copy.clone())
    }

    fn send(&self, o: ::grpc::RequestOptions, p: super::core::SendOptions) -> ::grpc::StreamingResponse<super::core::SendMessage> {
        self.grpc_client.call_server_streaming(o, p, self.method_Send.clone())
    }

    fn build_image(&self, o: ::grpc::RequestOptions, p: super::core::BuildImageOptions) -> ::grpc::StreamingResponse<super::core::BuildImageMessage> {
        self.grpc_client.call_server_streaming(o, p, self.method_BuildImage.clone())
    }

    fn cache_image(&self, o: ::grpc::RequestOptions, p: super::core::CacheImageOptions) -> ::grpc::StreamingResponse<super::core::CacheImageMessage> {
        self.grpc_client.call_server_streaming(o, p, self.method_CacheImage.clone())
    }

    fn remove_image(&self, o: ::grpc::RequestOptions, p: super::core::RemoveImageOptions) -> ::grpc::StreamingResponse<super::core::RemoveImageMessage> {
        self.grpc_client.call_server_streaming(o, p, self.method_RemoveImage.clone())
    }

    fn create_container(&self, o: ::grpc::RequestOptions, p: super::core::DeployOptions) -> ::grpc::StreamingResponse<super::core::CreateContainerMessage> {
        self.grpc_client.call_server_streaming(o, p, self.method_CreateContainer.clone())
    }

    fn replace_container(&self, o: ::grpc::RequestOptions, p: super::core::ReplaceOptions) -> ::grpc::StreamingResponse<super::core::ReplaceContainerMessage> {
        self.grpc_client.call_server_streaming(o, p, self.method_ReplaceContainer.clone())
    }

    fn remove_container(&self, o: ::grpc::RequestOptions, p: super::core::RemoveContainerOptions) -> ::grpc::StreamingResponse<super::core::RemoveContainerMessage> {
        self.grpc_client.call_server_streaming(o, p, self.method_RemoveContainer.clone())
    }

    fn dissociate_container(&self, o: ::grpc::RequestOptions, p: super::core::DissociateContainerOptions) -> ::grpc::StreamingResponse<super::core::DissociateContainerMessage> {
        self.grpc_client.call_server_streaming(o, p, self.method_DissociateContainer.clone())
    }

    fn control_container(&self, o: ::grpc::RequestOptions, p: super::core::ControlContainerOptions) -> ::grpc::StreamingResponse<super::core::ControlContainerMessage> {
        self.grpc_client.call_server_streaming(o, p, self.method_ControlContainer.clone())
    }

    fn realloc_resource(&self, o: ::grpc::RequestOptions, p: super::core::ReallocOptions) -> ::grpc::StreamingResponse<super::core::ReallocResourceMessage> {
        self.grpc_client.call_server_streaming(o, p, self.method_ReallocResource.clone())
    }

    fn log_stream(&self, o: ::grpc::RequestOptions, p: super::core::ContainerID) -> ::grpc::StreamingResponse<super::core::LogStreamMessage> {
        self.grpc_client.call_server_streaming(o, p, self.method_LogStream.clone())
    }

    fn run_and_wait(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::core::RunAndWaitOptions>) -> ::grpc::StreamingResponse<super::core::AttachContainerMessage> {
        self.grpc_client.call_bidi(o, p, self.method_RunAndWait.clone())
    }

    fn execute_container(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::core::ExecuteContainerOptions>) -> ::grpc::StreamingResponse<super::core::AttachContainerMessage> {
        self.grpc_client.call_bidi(o, p, self.method_ExecuteContainer.clone())
    }
}

// server

pub struct CoreRPCServer;


impl CoreRPCServer {
    pub fn new_service_def<H : CoreRPC + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/pb.CoreRPC",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/ListNetworks".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_networks(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/ListPods".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_pods(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/AddPod".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.add_pod(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/RemovePod".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.remove_pod(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/GetPod".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_pod(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/GetPodResource".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_pod_resource(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/ListPodNodes".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_pod_nodes(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/AddNode".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.add_node(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/RemoveNode".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.remove_node(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/SetNode".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.set_node(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/GetNode".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_node(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/GetNodeResource".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_node_resource(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/GetContainer".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_container(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/GetContainers".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_containers(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/ListContainers".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.list_containers(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/ListNodeContainers".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_node_containers(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/GetContainersStatus".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_containers_status(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/SetContainersStatus".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.set_containers_status(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/ContainerStatusStream".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.container_status_stream(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/Copy".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.copy(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/Send".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.send(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/BuildImage".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.build_image(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/CacheImage".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.cache_image(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/RemoveImage".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.remove_image(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/CreateContainer".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.create_container(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/ReplaceContainer".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.replace_container(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/RemoveContainer".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.remove_container(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/DissociateContainer".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.dissociate_container(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/ControlContainer".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.control_container(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/ReallocResource".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.realloc_resource(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/LogStream".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.log_stream(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/RunAndWait".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Bidi,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerBidi::new(move |o, p| handler_copy.run_and_wait(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/pb.CoreRPC/ExecuteContainer".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Bidi,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerBidi::new(move |o, p| handler_copy.execute_container(o, p))
                    },
                ),
            ],
        )
    }
}
