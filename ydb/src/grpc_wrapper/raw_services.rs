use strum::{Display, EnumIter, EnumString};
use ydb_grpc::ydb_proto::query::v1::query_service_client::QueryServiceClient;
use ydb_grpc::ydb_proto::table::v1::table_service_client::TableServiceClient;

pub(crate) trait GrpcServiceForDiscovery {
    fn get_grpc_discovery_service() -> Service;
}

impl<T> GrpcServiceForDiscovery for TableServiceClient<T> {
    fn get_grpc_discovery_service() -> Service {
        Service::Table
    }
}
impl<T> GrpcServiceForDiscovery for QueryServiceClient<T> {
    fn get_grpc_discovery_service() -> Service {
        Service::Query
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Display, Debug, EnumIter, EnumString, Eq, Hash, PartialEq)]
pub(crate) enum Service {
    #[strum(serialize = "discovery")]
    Discovery,

    #[strum(serialize = "export")]
    Export,

    #[strum(serialize = "import")]
    Import,

    #[strum(serialize = "scripting")]
    Scripting,

    #[strum(serialize = "table_service")]
    Table,

    #[strum(serialize = "scheme_service")]
    Scheme,

    #[strum(serialize = "topic_service")]
    Topic,

    #[strum(serialize = "coordination_service")]
    Coordination,

    #[strum(serialize = "auth_service")]
    Auth,

    #[strum(serialize = "query_service")]
    Query,
}
