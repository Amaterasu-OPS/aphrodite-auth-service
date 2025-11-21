use crate::utils::api_response::{ApiError, ApiSuccess};

pub trait UseCaseInterface {
    type Request;
    type Response;
    async fn handle(&self, data: Self::Request) -> Result<ApiSuccess<Self::Response>, ApiError>;
}