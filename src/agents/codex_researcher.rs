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

    /// Record a failed mnemonic attempt along with metrics.
    pub fn record_failure(
        &self,
        mnemonic: &str,
        partial_zpub: &str,
        entropy: &[u8],
        deviation: f64,
    ) -> std::io::Result<()> {
        let score = entropy_score(entropy);
        let hints = derive_heuristics(partial_zpub);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)?;

        writeln!(
            file,
            "## Failed Reconstruction\n- Mnemonic: `{}`\n- Partial zpub: `{}`\n- Entropy score: {:.2}\n- Deviation: {:.2}\n### Heuristics",
            mnemonic,
            partial_zpub,
            score,
            deviation
        )?;
        for h in hints {
            writeln!(file, "- {}", h)?;
        }
        writeln!(file)?;
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
            let p = c as f64 / len;
            score -= p * p.log2();
        }
    }
    score
}

fn derive_heuristics(partial_zpub: &str) -> Vec<String> {
    let mut hints = Vec::new();
    if partial_zpub.len() < 6 {
        hints.push("try deeper prefix search".to_string());
    }
    if !partial_zpub.starts_with('z') {
        hints.push("check BIP84 path or prefix".to_string());
    }
    hints
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn writes_markdown_output() {
        let tmp =
            std::env::temp_dir().join(format!("codex-test-{}.md", rand::thread_rng().gen::<u32>()));
        let agent = CodexResearcher::new(&tmp);
        agent
            .record_failure("alpha beta gamma", "zpub1234", &[0, 1, 2, 3, 4], 0.5)
            .unwrap();
        let contents = std::fs::read_to_string(&tmp).unwrap();
        assert!(contents.contains("Failed Reconstruction"));
        let _ = std::fs::remove_file(tmp);
    }
}
