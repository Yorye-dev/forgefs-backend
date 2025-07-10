use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UploadFile {
    pub name: String,
    pub data: String,
    //pub path: String,
    pub size: usize,
}
