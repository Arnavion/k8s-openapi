/// The type of errors returned by the Kubernetes API functions that prepare the HTTP request.
#[derive(Debug)]
pub enum RequestError {
    /// An error from preparing the HTTP request.
    Http(http::Error),

    /// An error while serializing a value into the JSON body of the HTTP request.
    Json(serde_json::Error),
}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestError::Http(err) => write!(f, "{err}"),
            RequestError::Json(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for RequestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RequestError::Http(err) => Some(err),
            RequestError::Json(err) => Some(err),
        }
    }
}

/// A trait implemented by all response types corresponding to Kubernetes API functions.
pub trait Response: Sized {
    /// Tries to parse the response from the given status code and response body.
    ///
    /// If an instance of `Self` can be successfully parsed from the given byte buffer, the instance is returned,
    /// along with the number of bytes used up from the buffer. Remove those bytes from the buffer before calling
    /// this function again.
    ///
    /// If the buffer does not contain enough bytes to be able to parse an instance of `Self`, the function returns
    /// `Err(ResponseError::NeedMoreData)`. Append more bytes into the buffer, then call this function again.
    ///
    /// Also see the [`ResponseBody`] type.
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ResponseError>;
}

/// This struct provides an easy way to parse a byte buffer into a Kubernetes API function's response.
///
/// All API function responses implement the [`Response`] trait, and are constructed by calling their [`Response::try_from_parts`] function.
/// If this function returns `Err(ResponseError::NeedMoreData)`, that means more bytes need to be appended to the function. Alternatively,
/// if the function returns `Ok((value, num_bytes_read))`, then `num_bytes_read` bytes need to be popped off from the front of the buffer.
///
/// The `ResponseBody` struct contains an internal dynamic buffer, and provides `append_slice` and `parse` functions to help with this.
/// `append_slice` appends the slice you give it to its internal buffer, and `parse` uses the [`Response::try_from_parts`] function to parse
/// the response out of the buffer, and truncates it accordingly.
///
/// You do not *have* to use this type to parse the response, say if you want to manage your own byte buffers. You can use
/// `<T as Response>::try_from_parts` directly instead.
pub struct ResponseBody<T> {
    /// The HTTP status code of the response.
    pub status_code: http::StatusCode,

    buf: bytes::BytesMut,

    _response: std::marker::PhantomData<fn() -> T>,
}

impl<T> ResponseBody<T> where T: Response {
    /// Construct a value for a response that has the specified HTTP status code.
    pub fn new(status_code: http::StatusCode) -> Self {
        ResponseBody {
            status_code,
            buf: Default::default(),
            _response: Default::default(),
        }
    }

    /// Append a slice of data from the HTTP response to this buffer.
    pub fn append_slice(&mut self, buf: &[u8]) {
        self.buf.extend_from_slice(buf);
    }

    /// Try to parse all the data buffered so far into a response type.
    pub fn parse(&mut self) -> Result<T, ResponseError> {
        match T::try_from_parts(self.status_code, &self.buf) {
            Ok((result, read)) => {
                self.advance(read);
                Ok(result)
            },

            Err(err) => Err(err),
        }
    }

    /// Drop the first `cnt` bytes of this buffer.
    ///
    /// This is useful for skipping over malformed bytes, such as invalid utf-8 sequences when parsing streaming `String` responses
    /// like from [`api::core::v1::Pod::read_log`](crate::api::core::v1::Pod::read_log).
    ///
    /// # Panics
    ///
    /// This function panics if `cnt` is greater than the length of the internal buffer.
    pub fn advance(&mut self, cnt: usize) {
        bytes::Buf::advance(&mut self.buf, cnt);
    }
}

impl<T> std::ops::Deref for ResponseBody<T> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}

/// The type of errors from parsing an HTTP response as one of the Kubernetes API functions' response types.
#[derive(Debug)]
pub enum ResponseError {
    /// An error from deserializing the HTTP response, indicating more data is needed to complete deserialization.
    NeedMoreData,

    /// An error while deserializing the HTTP response as a JSON value, indicating the response is malformed.
    Json(serde_json::Error),

    /// An error while deserializing the HTTP response as a string, indicating that the response data is not UTF-8.
    Utf8(std::str::Utf8Error),
}

impl std::fmt::Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseError::NeedMoreData => f.write_str("need more response data"),
            ResponseError::Json(err) => write!(f, "{err}"),
            ResponseError::Utf8(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for ResponseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ResponseError::NeedMoreData => None,
            ResponseError::Json(err) => Some(err),
            ResponseError::Utf8(err) => Some(err),
        }
    }
}

/// Extensions to the percent-encoding crate
pub mod percent_encoding2 {
    /// Ref <https://url.spec.whatwg.org/#path-percent-encode-set>
    pub const PATH_SEGMENT_ENCODE_SET: &percent_encoding::AsciiSet =
        &percent_encoding::CONTROLS
        .add(b' ').add(b'"').add(b'<').add(b'>').add(b'`') // fragment percent-encode set
        .add(b'#').add(b'?').add(b'{').add(b'}'); // path percent-encode set
}
