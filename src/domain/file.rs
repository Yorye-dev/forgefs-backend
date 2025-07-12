use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub struct UploadFile {
    pub name: String,
    pub data: String,
    //pub path: String,
    pub size: usize,
}

#[derive(Deserialize)]
pub struct UploadQuery {
    pub dest_path: String,
}
