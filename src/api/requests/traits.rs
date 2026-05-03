use crate::api::responses::Response;

pub trait IntoResponse {
    fn into_response(&self) -> Response;
}
