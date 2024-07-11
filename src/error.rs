use thiserror::Error;

#[derive(Error, Debug)]
pub enum RowError {
    #[error("Data type error: {0}")]
    DataTypeError(String),
    #[error("Conversion error: {0}")]
    ConvertError(#[from] serde_yaml::Error),
}
