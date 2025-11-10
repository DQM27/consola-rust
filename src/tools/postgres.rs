// src/tools/postgres.rs
use crate::config::DevEnvManager;
use crate::path;
use std::path::PathBuf;

#[derive(Clone)]
pub struct EnvironmentConfig {
    pub path: PathBuf,
    pub bin_dir: PathBuf,
}

pub fn setup(manager: &DevEnvManager) {
    println!("Configurando PostgreSQL...");
    let config = if let Some(saved) = manager.load_config(".postgres-config") {
        println!("Configuración existente: {}", saved.display());
        Some(EnvironmentConfig {
            path: saved.clone(),
            bin_dir: saved.join("bin"),
        })
    } else {
        find_postgres(manager).or_else(|| {
            println!("No encontrado automáticamente");
            manager.request_manual_path("PostgreSQL", "bin\\postgres.exe")
                .map(|p| EnvironmentConfig { path: p.clone(), bin_dir: p.join("bin") })
        })
    };

    if let Some(cfg) = config {
        let _ = manager.save_config(".postgres-config", &cfg.path);
        let _ = path::add_to_user_path(&cfg.bin_dir);
        println!("Ejecuta: devenv --init-postgres");
    } else {
        println!("No se pudo configurar PostgreSQL");
    }
}

fn find_postgres(manager: &DevEnvManager) -> Option<EnvironmentConfig> {
    let paths = vec![
        manager.config_dir.clone(),
        manager.config_dir.join("pgsql"),
        "C:\\pgsql".into(),
        "C:\\Program Files\\PostgreSQL".into(),
    ];
    for p in paths {
        let exe = p.join("bin").join("postgres.exe");
        if exe.exists() {
            return Some(EnvironmentConfig { path: p.clone(), bin_dir: p.join("bin") });
        }
    }
    None
}