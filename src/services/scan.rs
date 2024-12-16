use crate::services::empty;
use http_body_util::combinators::BoxBody;
use http_body_util::BodyExt;
use hyper::body::{Body, Bytes};
use hyper::{Method, Request, Response, Result, StatusCode};
use log::{error, info};
use std::pin::Pin;
use std::task::{Context, Poll};

/// Receives location barcode and labware, scans them into LabWhere.
pub async fn scan(
    req: Request<impl Body<Data = Bytes, Error = hyper::Error> + Send + Sync + 'static>,
) -> std::result::Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
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

struct MockBody {
    data: &'static [u8],
}

impl MockBody {
    fn new(data: &'static [u8]) -> Self {
        Self { data }
    }
}

unsafe impl Send for MockBody {}

impl Body for MockBody {
    type Data = Bytes;
    type Error = hyper::Error;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Result<http_body::Frame<Bytes>>>> {
        if self.data.is_empty() {
            Poll::Ready(None)
        } else {
            let data = self.data;
            self.data = &[];
            Poll::Ready(Some(Ok(http_body::Frame::data(Bytes::from(data)))))
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::services::scan::MockBody;

    #[tokio::test]
    async fn test_scan() {
        let body: MockBody = MockBody::new(b"anything");
        let req = hyper::Request::builder()
            .method("POST")
            .uri("/scan")
            .body(body)
            .unwrap();
        let res = super::scan(req).await.unwrap();
        assert_eq!(res.status(), 200);
    }
}
