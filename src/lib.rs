pub mod config;
pub mod entropy;
pub mod shamir;
pub mod bip39;
pub mod search;
pub mod agents;
pub mod utils;
pub mod ui;

pub fn run() {
    ui::init_global();
    println!("[+] MSRS Core Logic Initialized.");
    ui::global().report();
}
