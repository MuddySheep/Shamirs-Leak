#[derive(Clone)]
pub struct Config {
    pub threads: usize,
    pub max_depth: usize,
    pub share1: String,
    pub share2: String,
    pub zpub: Option<String>,
    pub progress: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            threads: 8,
            max_depth: 1024,
            share1: String::new(),
            share2: String::new(),
            zpub: None,
            progress: false,
        }
    }
}
