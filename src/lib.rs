pub mod config;
pub mod entropy;
pub mod shamir;
pub mod bip39;
pub mod search;
pub mod agents;
pub mod utils;
pub mod ui;
pub mod cli;
use crate::utils::decode_share_mnemonic;
const _: () = {
    #[cfg(not(target_endian = "little"))]
    compile_error!("msrs assumes little-endian");
};

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

    let prng_cfg = entropy::prng::PrngSettings { reuse_period: cfg.prng_reuse_period, mask: cfg.prng_mask };
    let result = search::brute::brute_force_third_share(
        &s1,
        &s2,
        zpub,
        cfg.max_depth,
        Some(&researcher),
        &prng_cfg,
        cfg.index_collision_prob,
        cfg.progress,
    );


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
