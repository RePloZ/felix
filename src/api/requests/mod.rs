mod body;
mod header;
mod traits;

use crate::api::responses::{
    ApiVersionsBody, DescribeTopicBody, ResHeaderV0, ResHeaderV1, Response, ResponseBody,
    ResponseHeader,
};
use crate::protocol::reader::StreamReader;
use tokio::io::AsyncReadExt;

pub use self::{
    body::api_versions::ApiVersionsRequest,
    body::describe_topics_partitions::ReqDescribeTopicPartitions, header::RequestHeader,
    traits::IntoResponse,
};

#[derive(Debug)]
enum RequestBody {
    ApiVersion(body::api_versions::ApiVersionsRequest),
    DescribeTopicPartitions(body::describe_topics_partitions::ReqDescribeTopicPartitions),
}

#[derive(Debug)]
pub struct Request {
    header: RequestHeader,
    body: RequestBody,
}

impl StreamReader for Request {
    async fn from_stream<R: AsyncReadExt + Unpin>(stream: &mut R) -> crate::error::Result<Self> {
        let header = RequestHeader::from_stream(stream).await?;
        println!("[Request] -> header: {header:#?}\n");

        let body = match header.key.0 {
            18 => RequestBody::ApiVersion(ApiVersionsRequest::from_stream(stream).await?),
            _ => RequestBody::DescribeTopicPartitions(
                ReqDescribeTopicPartitions::from_stream(stream).await?,
            ),
        };
        println!("[Request] -> body: {body:#?}\n");

        Ok(Self { header, body })
    }
}

impl IntoResponse for Request {
    fn into_response(&self) -> Response {
        match &self.body {
            RequestBody::ApiVersion(api_versions_request) => {
                let header = ResHeaderV0::from_request(&self.header);
                let body = ApiVersionsBody::from_request(&self.header, &api_versions_request);
                Response::ApiVersions(header, body)
            }
            RequestBody::DescribeTopicPartitions(req_describe_topic_partitions) => {
                let header = ResHeaderV1::from_request(&self.header);
                println!("[Response] -> header: {header:#?}\n");
                let body =
                    DescribeTopicBody::from_request(&self.header, &req_describe_topic_partitions);
                println!("[Response] -> body: {body:#?}");
                Response::DescribeTopic(header, body)
            }
        }
    }
}
