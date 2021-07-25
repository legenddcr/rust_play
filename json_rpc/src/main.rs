use jsonrpc_core::*;
use jsonrpc_derive::rpc;

// #[cfg(test)]

#[rpc]
pub trait Rpc {
	/// Adds two numbers and returns a result
	#[rpc(name = "add")]
	fn add(&self, u64, u64) -> Result<u64>;
}

pub struct RpcImpl;
impl Rpc for RpcImpl {
	fn add(&self, a: u64, b: u64) -> Result<u64> {
		Ok(a + b)
	}
}

#[test]
fn it_works() {
	let mut io = jsonrpc_core::IoHandler::new();
	io.extend_with(RpcImpl.to_delegate())
}

#[test]
fn basic_test() {
    let mut io = IoHandler::new();
    io.add_sync_method("say_hello", |_| {
       Ok(Value::String("Hello World!".into())) 
    });

    let request = r#"{"jsonrpc": "2.0", "method": "say_hello", "params": [42, 23], "id": 1}"#;
    let response = r#"{"jsonrpc":"2.0","result":"Hello World!","id":1}"#;
    
    assert_eq!(io.handle_request_sync(request), Some(response.to_string()));
}

fn main() {
    let mut io = IoHandler::new();
    io.add_sync_method("say_hello", |_| {
       Ok(Value::String("Hello world!".into())) 
    });

    let request = r#"{"jsonrpc": "2.0", "method": "say_hello", "params": [42, 23], "id": 1}"#;
    let response = r#"{"jsonrpc":"2.0","result":"Hello World!","id":1}"#;
    
    assert_eq!(io.handle_request_sync(request), Some(response.to_string()));
}