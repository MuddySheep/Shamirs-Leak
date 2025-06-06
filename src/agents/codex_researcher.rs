use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

// compile-time endianness check
const _: () = {
    #[cfg(not(target_endian = "little"))]
    compile_error!("codex_researcher assumes little-endian target");
};

pub struct CodexResearcher {
    log_path: PathBuf,
}

impl CodexResearcher {
    /// Create a new researcher that writes to `path`.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            log_path: path.as_ref().to_path_buf(),
        }
    }

    /// Record an attempted mnemonic reconstruction with metrics.
    pub fn record_attempt(
        &self,
        mnemonic: &str,
        candidate_zpub: &str,
        target_zpub: &str,
        entropy: &[u8],
        path: &str,
        score: f64,
    ) -> std::io::Result<()> {
        let entropy_score = entropy_score(entropy);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)?;

        writeln!(
            file,
            "## Attempt\n- Mnemonic: `{}`\n- Candidate zpub: `{}`\n- Target zpub: `{}`\n- Score: {:.4}\n- Entropy score: {:.2}\n- Path: `{}`\n",
            mnemonic,
            candidate_zpub,
            target_zpub,
            score,
            entropy_score,
            path
        )?;
        Ok(())
    }
}

fn entropy_score(data: &[u8]) -> f64 {
    let mut counts = [0u32; 256];
    for &b in data {
        counts[b as usize] += 1;
    }
    let len = data.len() as f64;
    if len == 0.0 {
        return 0.0;
    }
    let mut score = 0.0;
    for &c in &counts {
        if c > 0 {
            let p = c as f64 / len as f64;
            score -= p * p.log2();
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn writes_markdown_output() {
        let tmp = std::env::temp_dir().join(format!("codex-test-{}.md", rand::thread_rng().gen::<u32>()));
        let agent = CodexResearcher::new(&tmp);
        agent
            .record_attempt("alpha beta gamma", "zpub1234", "zpubXYZ", &[0, 1, 2, 3, 4], "1/2/3", 0.5)
            .unwrap();
        let contents = std::fs::read_to_string(&tmp).unwrap();
        assert!(contents.contains("Candidate zpub"));
        let _ = std::fs::remove_file(tmp);
    }
}
