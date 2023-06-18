///
#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    /// The key can only contain alphanumeric characters and `_`.
    #[error("invalid key")]
    InvalidKey,
    /// The key can only contain alphanumeric characters and `_`.
    #[error("io error")]
    IoError(#[from] std::io::Error),
    /// The key can only contain alphanumeric characters and `_`.
    #[error("serialization error: {0}")]
    SerializationError(String),
}

///
#[derive(Clone)]
pub struct Cache {
    path: std::path::PathBuf,
}

impl Cache {
    pub fn new(path: &std::path::Path) -> Self {
        Self {
            path: path.to_owned(),
        }
    }

    ///
    pub fn save_config<T>(&self, key: &str, value: &T) -> Result<(), CacheError>
    where
        T: serde::Serialize,
    {
        if !key.chars().all(|c| c.is_alphanumeric() || (c == '_')) {
            return Err(CacheError::InvalidKey);
        }

        let mut path = self.path.clone();
        path.push(key);

        let file = std::fs::File::options()
            .create(true)
            .write(true)
            .open(&path)?;

        let writer = std::io::BufWriter::new(file);
        // serde_json::to_writer_pretty(writer, value)
        //     .map_err(|e| CacheError::SerializationError(e.to_string()))?;

        Ok(())
    }
}
