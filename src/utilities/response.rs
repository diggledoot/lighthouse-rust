use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub data: Vec<T>,
    pub message: String,
}
