use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestPayload<T1, T2> {
    #[serde(rename = "jsonrpc")]
    pub json_rpc: String,
    pub id: u32,
    pub method: String,
    pub params: (T1, T2),
}

impl<T1, T2> RequestPayload<T1, T2> {
    pub fn new(
        json_rpc: String,
        id: u32,
        method: String,
        params: (T1, T2),
    ) -> RequestPayload<T1, T2> {
        RequestPayload {
            json_rpc,
            id,
            method,
            params,
        }
    }
}
