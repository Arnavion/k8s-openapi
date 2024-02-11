use k8s_openapi::serde_json;

mod create;
pub(crate) use create::{create_cluster, create_namespaced, CreateResponse};

mod delete;
pub(crate) use delete::{delete_cluster, delete_namespaced, DeleteResponse};

mod delete_collection;
pub(crate) use delete_collection::{delete_collection_namespaced, ListOptional};

mod list;
pub(crate) use list::{list_namespaced, ListResponse};

mod patch;
pub(crate) use patch::{patch_namespaced, PatchResponse};

mod read;
pub(crate) use read::{read_cluster, read_namespaced, ReadResponse};

mod watch;
pub(crate) use watch::{watch_namespaced, WatchResponse};
#[allow(unused_imports)]
pub(crate) use watch::WatchOptional;

pub(crate) struct ResponseBody<T> {
    pub(crate) status_code: reqwest::StatusCode,
    buf: bytes::BytesMut,
    _response: std::marker::PhantomData<fn() -> T>,
}

impl<T> ResponseBody<T> where T: Response {
    pub(crate) fn new(status_code: reqwest::StatusCode) -> Self {
        ResponseBody {
            status_code,
            buf: Default::default(),
            _response: Default::default(),
        }
    }

    pub(crate) fn append_slice(&mut self, buf: &[u8]) {
        self.buf.extend_from_slice(buf);
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    pub(crate) fn parse(&mut self) -> Result<T, ResponseError> {
        match T::try_from_parts(self.status_code, &self.buf) {
            Ok((result, read)) => {
                self.advance(read);
                Ok(result)
            },

            Err(err) => Err(err),
        }
    }

    pub(crate) fn advance(&mut self, cnt: usize) {
        bytes::Buf::advance(&mut self.buf, cnt);
    }
}

impl<T> std::ops::Deref for ResponseBody<T> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}

pub(crate) trait Response: Sized {
    fn try_from_parts(status_code: reqwest::StatusCode, buf: &[u8]) -> Result<(Self, usize), ResponseError>;
}

#[derive(Debug)]
pub enum ResponseError {
    NeedMoreData,
    Json(serde_json::Error),
}

impl std::fmt::Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseError::NeedMoreData => f.write_str("need more response data"),
            ResponseError::Json(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for ResponseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ResponseError::NeedMoreData => None,
            ResponseError::Json(err) => Some(err),
        }
    }
}

pub(crate) fn get_api_versions() -> (http::Request<Vec<u8>>, fn(reqwest::StatusCode) -> ResponseBody<GetAPIVersionsResponse>) {
    let url = "/apis".to_owned();

    let request = http::Request::get(url);
    let request = request.body(vec![]).unwrap();
    (request, ResponseBody::new)
}

#[derive(Debug)]
pub(crate) enum GetAPIVersionsResponse {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroupList),
    Other(#[allow(dead_code)] Result<Option<serde_json::Value>, serde_json::Error>),
}

impl Response for GetAPIVersionsResponse {
    fn try_from_parts(status_code: reqwest::StatusCode, buf: &[u8]) -> Result<(Self, usize), ResponseError> {
        #[allow(clippy::single_match_else)]
        match status_code {
            reqwest::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(ResponseError::NeedMoreData),
                    Err(err) => return Err(ResponseError::Json(err)),
                };
                Ok((Self::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((Self::Other(result), read))
            },
        }
    }
}

/// Ref <https://url.spec.whatwg.org/#path-percent-encode-set>
const PATH_SEGMENT_ENCODE_SET: &percent_encoding::AsciiSet =
    &percent_encoding::CONTROLS
    .add(b' ').add(b'"').add(b'<').add(b'>').add(b'`') // fragment percent-encode set
    .add(b'#').add(b'?').add(b'{').add(b'}'); // path percent-encode set
