use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt};
use hyper::body::Bytes;
use hyper::Method;
use hyper::{Request, Response, StatusCode};
use log::{error, info};
use crate::services::empty;

/// Receives location barcode and labware, scans them into LabWhere.
pub async fn scan(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    info!("Processing request for /scan endpoint");
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/scan") => Ok(Response::new(req.into_body().boxed())),
        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            error!("Responding with not found");
            Ok(not_found)
        }
    }
}
