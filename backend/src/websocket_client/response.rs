use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RpcResponse<T> {
    pub jsonrpc: String,
    pub result: T,
    pub id: u32,
}
