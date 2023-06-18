use directories::ProjectDirs;

///
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
struct ConfigInner {
    proxy: Option<String>,
    cache_path: Option<std::path::PathBuf>,
}

///
impl Default for ConfigInner {
    fn default() -> Self {
        Self {
            proxy: None,
            cache_path: None,
        }
    }
}

///
pub struct Config {
    inner: parking_lot::RwLock<ConfigInner>,
    default_cache_path: std::path::PathBuf,
    config_path: std::path::PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let project_dirs = ProjectDirs::from("com", "fukurou", "fukurou").unwrap();
        let default_cache_path = project_dirs.cache_dir().to_owned();

        let mut config_path = project_dirs.config_dir().to_owned();
        config_path.push("config.json");

        // let file = std::fs::File::options()
        //     .create(true)
        //     .write(true)
        //     .open(&config_path)
        //     .unwrap();

        // let reader = std::io::BufReader::new(file);
        // let inner: ConfigInner = serde_json::from_reader(reader).unwrap();
        let inner = ConfigInner::default();

        Self {
            inner: parking_lot::RwLock::new(inner),
            default_cache_path,
            config_path,
        }
    }

    ///
    pub fn proxy(&self) -> Option<String> {
        self.inner.read().proxy.clone()
    }

    ///
    pub fn set_proxy(&self, proxy: Option<String>) {
        self.inner.write().proxy = proxy;
    }

    ///
    pub fn cache_path(&self) -> std::path::PathBuf {
        let path = self.inner.read().cache_path.clone();
        path.unwrap_or_else(|| self.default_cache_path.clone())
    }

    ///
    pub fn set_cache_path(&self, cache_path: Option<std::path::PathBuf>) {
        self.inner.write().cache_path = cache_path;
    }
}

///
impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

///
impl Drop for Config {
    fn drop(&mut self) {
        let file = std::fs::File::options()
            .create(true)
            .write(true)
            .open(&self.config_path)
            .unwrap();

        let writer = std::io::BufWriter::new(file);
        let inner = self.inner.read();
        // serde_json::to_writer_pretty(writer, &*inner).unwrap()
    }
}
