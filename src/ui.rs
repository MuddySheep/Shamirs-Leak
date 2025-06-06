use std::sync::{atomic::{AtomicU64, Ordering}, Mutex, Arc, OnceLock};
use std::time::{Instant, Duration};
use rayon::current_num_threads;

// compile-time assertion: require 64-bit target for atomic counters
const _: () = {
    #[cfg(target_pointer_width = "32")]
    compile_error!("ui::Stats requires 64-bit target");
};

// compile-time check for AtomicU64 size
const _: [(); 8] = [(); std::mem::size_of::<AtomicU64>()];

pub struct Stats {
    total_candidates: AtomicU64,
    mnemonic_matches: AtomicU64,
    best_score: AtomicU64,
    best_prefix: Mutex<String>,
    start: Instant,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            total_candidates: AtomicU64::new(0),
            mnemonic_matches: AtomicU64::new(0),
            best_score: AtomicU64::new(0),
            best_prefix: Mutex::new(String::new()),
            start: Instant::now(),
        }
    }

    pub fn inc_candidates(&self, n: u64) {
        self.total_candidates.fetch_add(n, Ordering::Relaxed);
    }

    pub fn inc_matches(&self, n: u64) {
        self.mnemonic_matches.fetch_add(n, Ordering::Relaxed);
    }

    pub fn update_best(&self, score: u64, prefix: &str) {
        let mut best = self.best_prefix.lock().unwrap();
        if score > self.best_score.load(Ordering::Relaxed) {
            self.best_score.store(score, Ordering::Relaxed);
            *best = prefix.to_string();
        }
    }

    pub fn report(&self) {
        let elapsed = self.start.elapsed().as_secs_f64();
        let total = self.total_candidates.load(Ordering::Relaxed);
        let matches = self.mnemonic_matches.load(Ordering::Relaxed);
        let best_score = self.best_score.load(Ordering::Relaxed);
        let best_prefix = self.best_prefix.lock().unwrap();
        let threads = current_num_threads();
        println!("+----------------------+-------------------------+");
        println!("| Metric               | Value                   |");
        println!("+----------------------+-------------------------+");
        println!("| Candidates tested    | {:>23} |", total);
        println!("| Mnemonic matches     | {:>23} |", matches);
        println!("| Best zpub score      | {:>23} |", best_score);
        println!("| Best zpub prefix     | {:>23} |", *best_prefix);
        if threads > 1 {
            let rate = if elapsed > 0.0 { total as f64 / elapsed } else { 0.0 };
            println!("| Rayon threads        | {:>23} |", threads);
            println!("| Candidates/sec       | {:>23.2} |", rate);
        }
        println!("+----------------------+-------------------------+");
    }
}

impl Drop for Stats {
    fn drop(&mut self) {
        self.report();
    }
}

static GLOBAL: OnceLock<Arc<Stats>> = OnceLock::new();

pub fn init_global() -> &'static Arc<Stats> {
    let stats = Arc::new(Stats::new());
    let _ = GLOBAL.set(Arc::clone(&stats));
    GLOBAL.get().unwrap()
}

pub fn global() -> &'static Arc<Stats> {
    GLOBAL.get().expect("Stats not initialized")
}

pub fn spawn_reporter(period: Duration) -> std::thread::JoinHandle<()> {
    let stats = Arc::clone(global());
    std::thread::spawn(move || loop {
        std::thread::sleep(period);
        stats.report();
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stats_basic_ops() {
        let s = Stats::new();
        s.inc_candidates(10);
        s.inc_matches(2);
        s.update_best(5, "abcd");
        assert_eq!(s.total_candidates.load(Ordering::Relaxed), 10);
        assert_eq!(s.mnemonic_matches.load(Ordering::Relaxed), 2);
        assert_eq!(s.best_score.load(Ordering::Relaxed), 5);
        assert_eq!(*s.best_prefix.lock().unwrap(), "abcd".to_string());
    }
}
