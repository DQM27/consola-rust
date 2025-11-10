// src/postgres/commands.rs
use crate::config::DevEnvManager;
use crate::tools::postgres::EnvironmentConfig;

pub fn run_with_config<F>(manager: &DevEnvManager, f: F)
where
    F: FnOnce(&EnvironmentConfig) -> std::io::Result<()>,
{
    if let Some(path) = manager.load_config(".postgres-config") {
        let cfg = EnvironmentConfig {
            path: path.clone(),
            bin_dir: path.join("bin"),
        };
        let _ = f(&cfg);
    } else {
        println!("PostgreSQL no configurado");
        println!("Ejecuta: devenv --setup-postgres");
    }
}