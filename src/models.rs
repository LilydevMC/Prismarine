use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub general: General
}

#[derive(Debug, Serialize, Deserialize)]
pub struct General {
    pub name: String,
    pub description: String,
    pub version: String,
    pub minecraft_version: String,
    pub license: String,
    pub name_template: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackMeta {
    pub pack: Pack
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pack {
    pub pack_format: i32,
    pub description: String
}

