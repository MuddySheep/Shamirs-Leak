pub mod agents;
pub mod bip39;
pub mod cli;
pub mod config;
pub mod entropy;
pub mod search;
pub mod shamir;
pub mod ui;
pub mod utils;
use crate::utils::decode_share_mnemonic;
pub use agents::*;
pub use bip39::{checksum, seed};
pub use search::brute;
pub use shamir::{gf256, reconstruct};
const _: () = {
    #[cfg(not(target_endian = "little"))]
    compile_error!("msrs assumes little-endian");
};

/// Execute the full agent pipeline given two known shares.
pub fn run_pipeline(
    share_a: &[u8],
    share_b: &[u8],
    expected_zpub: &str,
    cfg: &config::Config,
    log: Option<&agents::codex_researcher::CodexResearcher>,
) -> Option<(Vec<u8>, String)> {
    assert_eq!(share_a.len(), share_b.len(), "share length mismatch");
    let entropy_agent =
        agents::entropy_profiler::EntropyProfilerAgent::new(entropy::prng::PrngSettings {
            reuse_period: cfg.prng_reuse_period,
            mask: cfg.prng_mask,
        });
    brute::brute_force_third_share(
        share_a,
        share_b,
        expected_zpub,
        cfg.max_depth,
        log,
        &entropy_agent.settings(),
        cfg.index_collision_prob,
        cfg.progress,
    )
}

pub fn run(cfg: config::Config) {
    ui::init_global();
    println!("[+] MSRS Core Logic Initialized.");

    let share1_txt = cli::CliArgs::load_mnemonic(&cfg.share1).expect("share1");
    let share2_txt = cli::CliArgs::load_mnemonic(&cfg.share2).expect("share2");

    let (idx1, pay1) = decode_share_mnemonic(&share1_txt).expect("decode share1");
    let (idx2, pay2) = decode_share_mnemonic(&share2_txt).expect("decode share2");

    let mut s1 = Vec::with_capacity(pay1.len() + 1);
    s1.push(idx1);
    s1.extend_from_slice(&pay1);

    let mut s2 = Vec::with_capacity(pay2.len() + 1);
    s2.push(idx2);
    s2.extend_from_slice(&pay2);

    let zpub = cfg.zpub.as_deref().unwrap_or("");
    let researcher = agents::codex_researcher::CodexResearcher::new("codex-replay.md");

    let result = run_pipeline(&s1, &s2, zpub, &cfg, Some(&researcher));

    if let Some((_, mnemonic)) = result {
        let derived = bip39::seed::derive_seed_zpub(&mnemonic).unwrap_or_default();
        ui::global().report();
        println!("[!] SUCCESS: {}", mnemonic);
        println!("Derived zpub: {}", derived);
    } else {
        ui::global().report();
        println!("[!] No valid mnemonic found");
    }
}
