use serde_json::{Value};
/// A unique identifier attached to request RPCs.
type RequestId = u64;

/// Reads and parses RPC messages from a stream, maintaining an
/// internal buffer.
#[derive(Debug, Default)]
pub struct MessageReader(String);

/// An internal type used during initial JSON parsing.
///
/// Wraps an arbitrary JSON object, which may be any valid or invalid
/// RPC message. This allows initial parsing and response handling to
/// occur on the read thread. If the message looks like a request, it
/// is passed to the main thread for handling.
#[derive(Debug, Clone)]
pub struct RpcObject(pub Value);

#[derive(Debug, Clone, PartialEq)]
/// An RPC call, which may be either a notification or a request.
pub enum Call<N, R> {
    /// An id and an RPC Request
    Request(RequestId, R),
    /// An RPC Notification
    Notification(N),
    /// A malformed request: the request contained an id, but could
    /// not be parsed. The client will receive an error.
    InvalidRequest(RequestId),
}

impl RpcObject {
    /// Returns the 'id' of the underlying object, if present.
    pub fn get_id(&self) -> Option<RequestId> {
        self.0.get("id").and_then(Value::as_u64)
    }

    /// Returns the 'method' field of the underlying object, if present.
    pub fn get_method(&self) -> Option<&str> {
        self.0.get("method").and_then(Value::as_str)
    }

    /// Returns `true` if this object looks like an RPC response;
    /// that is, if it has an 'id' field and does _not_ have a 'method'
    /// field.
    pub fn is_response(&self) -> bool {
        self.0.get("id").is_some() && self.0.get("method").is_none()
    }
}

// convert a valid RPC json value into RpcObject
impl From<Value> for RpcObject {
    fn from(v: Value) -> RpcObject {
        RpcObject(v)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use serde_json;

    #[test]
    fn create_rpc_obj_with_json_str() {
        let json = r#"{"id":0,"method":"open_file","params":{}}"#;
        let p: RpcObject = serde_json::from_str::<Value>(json).unwrap().into();
        assert!(!p.is_response());
        assert_eq!(p.get_id(), Some(0));
        assert_eq!(p.get_method(), Some("open_file"));
    }
}
