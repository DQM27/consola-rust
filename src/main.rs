// src/main.rs
mod config;
mod tools;
mod postgres;
mod path;
mod script;
mod ui;

use config::DevEnvManager;
use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 { exit(0); }

    let manager = match DevEnvManager::new() {
        Ok(m) => m,
        Err(e) => { eprintln!("Error: {}", e); exit(1); }
    };

    match args[1].as_str() {
        "--DQM27" => ui::show_help(),
        "--setup-mingw" => tools::setup_mingw(&manager, &args),
        "--setup-postgres" => tools::setup_postgres(&manager, &args),
        "--setup-node" => tools::setup_node(&manager, &args),
        "--generate-script" => script::generate(&manager),
        "--verify" => ui::verify_tools(&manager),
        "--init-postgres" => postgres::init(&manager),
        "--start-postgres" => postgres::start(&manager),
        "--stop-postgres" => postgres::stop(&manager),
        "--restart-postgres" => postgres::restart(&manager),
        "--status-postgres" => postgres::status(&manager),
        "--list-dbs" => postgres::list_dbs(&manager),
        "--create-db" => postgres::create_db(&manager, &args),
        "--drop-db" => postgres::drop_db(&manager, &args),
        "--backup-db" => postgres::backup_db(&manager, &args),
        "--restore-db" => postgres::restore_db(&manager, &args),
        "--psql" => postgres::psql(&manager, &args),
        _ => exit(0),
    }
}