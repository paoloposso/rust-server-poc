use super::method::Method;

pub struct Request {
    path: String,
    querystring: Option<String>,
    method: Method,
}