use azure_storage::core::headers::CommonStorageResponseHeaders;
use bytes::Bytes;
use http::response::Response;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreateQueueResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl std::convert::TryFrom<&Response<Bytes>> for CreateQueueResponse {
    type Error = crate::Error;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        debug!("response == {:?}", response);

        Ok(CreateQueueResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
