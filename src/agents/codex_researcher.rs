use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use crate::search::diff::DiffMetrics;

// compile-time check on DiffMetrics size (24 bytes on 64-bit)
#[cfg(target_pointer_width = "64")]
const _: [(); 24] = [(); core::mem::size_of::<DiffMetrics>()];

// compile-time endianness check
const _: () = {
    #[cfg(not(target_endian = "little"))]
    compile_error!("codex_researcher assumes little-endian target");
};

pub struct CodexResearcher {
    log_path: PathBuf,
    csv_path: PathBuf,
    stats: Mutex<Vec<DiffMetrics>>, // why: accumulate metrics across attempts
}

impl CodexResearcher {
    /// Create a new researcher that writes to `path`.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let log_path = path.as_ref().to_path_buf();
        let csv_path = log_path.with_extension("csv");
        let mut csv = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&csv_path)
            .expect("create csv log");
        // why: header for chart-ready data
        writeln!(csv, "attempt,prefix_len,hamming_distance,similarity").unwrap();
        Self {
            log_path,
            csv_path,
            stats: Mutex::new(Vec::new()),
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
        diff: DiffMetrics,
    ) -> std::io::Result<()> {
        let entropy_score = entropy_score(entropy);

        let attempt_idx = {
            let stats = self.stats.lock().unwrap();
            stats.len() + 1
        };

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
            diff.similarity,
            entropy_score,
            path
        )?;

        let mut csv = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.csv_path)?;
        writeln!(
            csv,
            "{},{},{},{:.4}",
            attempt_idx,
            diff.prefix_len,
            diff.hamming_distance,
            diff.similarity
        )?;

        self.stats.lock().unwrap().push(diff);

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
        let diff = crate::search::diff::zpub_diff("zpubA", "zpubB");
        agent
            .record_attempt("alpha beta gamma", "zpubA", "zpubB", &[0, 1, 2, 3, 4], "1/2/3", diff)
            .unwrap();
        let contents = std::fs::read_to_string(&tmp).unwrap();
        assert!(contents.contains("Candidate zpub"));
        let csv_path = tmp.with_extension("csv");
        let csv = std::fs::read_to_string(&csv_path).unwrap();
        assert!(csv.lines().count() > 1);
        let _ = std::fs::remove_file(&tmp);
        let _ = std::fs::remove_file(csv_path);
    }
}
