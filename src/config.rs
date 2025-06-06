#[derive(Clone)]
pub struct Config {
    pub threads: usize,
    pub max_depth: usize,
    pub share1: String,
    pub share2: String,
    pub zpub: Option<String>,
    pub prng_reuse_period: usize,
    pub prng_mask: u8,
    pub index_collision_prob: f64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            threads: 8,
            max_depth: 1024,
            share1: String::new(),
            share2: String::new(),
            zpub: None,
            prng_reuse_period: 4,
            prng_mask: 0x7f,
            index_collision_prob: 0.0,
        }
    }
}
