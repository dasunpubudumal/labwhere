use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Empty};
use hyper::body::Bytes;

pub mod scan;

/// An empty function visible only to the crate scope that returns a
/// boxed empty response. This can be used for 404 error responses.
pub(crate) fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}
