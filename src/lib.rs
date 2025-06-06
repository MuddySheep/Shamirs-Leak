pub mod config;
pub mod entropy;
pub mod shamir;
pub mod bip39;
pub mod search;
pub mod agents;
pub mod utils;
pub mod ui;
pub mod cli;
use ::bip39::Mnemonic;
const _: () = {
    #[cfg(not(target_endian = "little"))]
    compile_error!("msrs assumes little-endian");
};

pub fn run(cfg: config::Config) {
    ui::init_global();
    println!("[+] MSRS Core Logic Initialized.");

    let share1_txt = cli::CliArgs::load_mnemonic(&cfg.share1).expect("share1");
    let share2_txt = cli::CliArgs::load_mnemonic(&cfg.share2).expect("share2");

    let m1 = Mnemonic::parse(&share1_txt).expect("mnemonic1");
    let m2 = Mnemonic::parse(&share2_txt).expect("mnemonic2");
    let s1 = m1.to_entropy().to_vec();
    let s2 = m2.to_entropy().to_vec();

    let zpub = cfg.zpub.as_deref().unwrap_or("");
    let researcher = agents::codex_researcher::CodexResearcher::new("codex-replay.md");
    let result = search::brute::brute_force_third_share(&s1, &s2, zpub, cfg.max_depth, Some(&researcher));

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
