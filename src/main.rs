fn main() {
    println!("🚀 Starting MSRS - Mnemonic Share Reverse Simulator");
    let args = msrs::cli::CliArgs::parse().expect("parse args");
    let mut cfg = msrs::config::Config::default();
    cfg.share1 = args.share1;
    cfg.share2 = args.share2;
    cfg.zpub = args.zpub;
    if let Some(t) = args.threads { cfg.threads = t; }
    if let Some(p) = args.prng_reuse { cfg.prng_reuse_period = p; }
    if let Some(m) = args.prng_mask { cfg.prng_mask = m; }
    if let Some(c) = args.index_collision { cfg.index_collision_prob = c; }
    msrs::run(cfg);
}
