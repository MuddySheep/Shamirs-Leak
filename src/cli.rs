use std::fs;
use std::path::Path;

/// Simple CLI argument holder.
#[derive(Debug, Clone)]
pub struct CliArgs {
    pub share1: String,
    pub share2: String,
    pub zpub: Option<String>,
    pub threads: Option<usize>,
    pub prng_reuse: Option<usize>,
    pub prng_mask: Option<u8>,
    pub index_collision: Option<f64>,
    pub progress: bool,
}

impl CliArgs {
    pub fn parse() -> Result<Self, String> {
        Self::parse_from(std::env::args().skip(1))
    }

    pub fn parse_from<I>(mut args: I) -> Result<Self, String>
    where
        I: Iterator<Item = String>,
    {
        // compile-time check: ensure usize is at least 8 bytes for thread counts
        const _: [(); 8] = [(); std::mem::size_of::<usize>()];

        let mut share1 = None;
        let mut share2 = None;
        let mut zpub = None;
        let mut threads = None;
        let mut prng_reuse = None;
        let mut prng_mask = None;
        let mut index_collision = None;
        let mut progress = false;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--share1" => share1 = args.next(),
                "--share2" => share2 = args.next(),
                "--zpub" => zpub = args.next(),
                "--threads" => {
                    if let Some(val) = args.next() {
                        threads = Some(val.parse().map_err(|e| format!("bad --threads: {}", e))?);
                    } else {
                        return Err("--threads requires value".into());
                    }
                }
                "--prng-reuse" => {
                    if let Some(val) = args.next() {
                        prng_reuse = Some(val.parse().map_err(|e| format!("bad --prng-reuse: {}", e))?);
                    } else {
                        return Err("--prng-reuse requires value".into());
                    }
                }
                "--prng-mask" => {
                    if let Some(val) = args.next() {
                        prng_mask = Some(val.parse().map_err(|e| format!("bad --prng-mask: {}", e))?);
                    } else {
                        return Err("--prng-mask requires value".into());
                    }
                }
                "--index-collision" => {
                    if let Some(val) = args.next() {
                        index_collision = Some(val.parse().map_err(|e| format!("bad --index-collision: {}", e))?);
                    } else {
                        return Err("--index-collision requires value".into());
                    }
                }
                "--progress" => {
                    progress = true;
                }
                other => return Err(format!("unknown arg: {}", other)),
            }
        }

        let share1 = share1.ok_or("--share1 required")?;
        let share2 = share2.ok_or("--share2 required")?;

        Ok(CliArgs {
            share1,
            share2,
            zpub,
            threads,
            prng_reuse,
            prng_mask,
            index_collision,
            progress,
        })
    }

    /// Load a mnemonic either from inline string or from file path.
    pub fn load_mnemonic(source: &str) -> Result<String, String> {
        if Path::new(source).exists() {
            fs::read_to_string(source)
                .map_err(|e| e.to_string())
                .map(|s| s.trim().to_string())
        } else {
            Ok(source.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_inline() {
        let args = vec!["--share1", "a", "--share2", "b"];
        let parsed = CliArgs::parse_from(args.into_iter().map(String::from)).unwrap();
        assert_eq!(parsed.share1, "a");
        assert_eq!(parsed.share2, "b");
        assert!(parsed.prng_reuse.is_none());
        assert!(parsed.prng_mask.is_none());
        assert!(parsed.index_collision.is_none());
        assert!(!parsed.progress);
    }
}
