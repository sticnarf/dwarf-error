use tokio::runtime::Builder;
use tokio::runtime::Runtime;
use tonic::codegen::*;
use tonic::IntoRequest;

#[derive(::prost::Message)]
pub struct GetRequest {}

#[derive(::prost::Message)]
pub struct GetResponse {}

fn main() {
    let future = async {
        let conn = tonic::transport::Endpoint::new("http://[::1]:50051")
            .unwrap()
            .connect()
            .await
            .unwrap();
        let mut client = tonic::client::Grpc::new(conn);
        let req = GetRequest::default();
        client.ready().await.unwrap();
        let codec = tonic::codec::ProstCodec::default();
        let path = http::uri::PathAndQuery::from_static("/tikvpb.Tikv/KvGet");
        let res: Result<tonic::Response<GetResponse>, tonic::Status> =
            client.unary(req.into_request(), path, codec).await;
    };
    let rt = Builder::new_current_thread().build().unwrap();
    rt.block_on(future);
}
