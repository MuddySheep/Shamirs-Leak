fn main() {
    println!("🚀 Starting MSRS - Mnemonic Share Reverse Simulator");
    let args = msrs::cli::CliArgs::parse().expect("parse args");
    let mut cfg = msrs::config::Config::default();
    cfg.share1 = args.share1;
    cfg.share2 = args.share2;
    cfg.zpub = args.zpub;
    cfg.progress = args.progress;
    if let Some(t) = args.threads { cfg.threads = t; }
    msrs::run(cfg);
}
