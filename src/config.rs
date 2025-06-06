pub struct Config {
    pub threads: usize,
    pub max_depth: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            threads: 8,
            max_depth: 1024,
        }
    }
}
