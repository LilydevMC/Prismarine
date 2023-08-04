use thiserror::Error;


#[derive(Debug, Error)]
pub enum PackError {
    #[error("Invalid Minecraft version.")]
    InvalidVersion
} 


