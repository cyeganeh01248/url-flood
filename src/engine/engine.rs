use super::request::Request;

pub struct Engine {
    request: Request,
    num_requests: Option<u32>,
}
