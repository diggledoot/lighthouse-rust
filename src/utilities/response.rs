#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Response<T> {
    pub data: Vec<T>,
    pub message: String,
}
