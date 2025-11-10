// src/tools/node.rs
use crate::config::DevEnvManager;
use crate::path;
use std::path::PathBuf;

#[derive(Clone)]
pub struct EnvironmentConfig {
    pub path: PathBuf,
    pub bin_dir: PathBuf,
}

pub fn setup(manager: &DevEnvManager) {
    println!("Configurando Node.js...");
    let config = if let Some(saved) = manager.load_config(".node-config") {
        println!("Configuración existente: {}", saved.display());
        Some(EnvironmentConfig {
            path: saved.clone(),
            bin_dir: saved.clone(),
        })
    } else {
        find_node(manager).or_else(|| {
            println!("No encontrado automáticamente");
            manager.request_manual_path("Node.js", "node.exe")
                .map(|p| EnvironmentConfig { path: p.clone(), bin_dir: p.clone() })
        })
    };

    if let Some(cfg) = config {
        let _ = manager.save_config(".node-config", &cfg.path);
        let _ = path::add_to_user_path(&cfg.bin_dir);
        println!("Ejecuta: devenv --generate-script");
    } else {
        println!("No se pudo configurar Node.js");
    }
}

fn find_node(_manager: &DevEnvManager) -> Option<EnvironmentConfig> {
    let paths: Vec<PathBuf> = vec![
        "C:\\Program Files\\nodejs".into(),
        "C:\\Program Files (x86)\\nodejs".into(),
    ];
    for p in paths {
        let exe = p.join("node.exe");
        if exe.exists() {
            return Some(EnvironmentConfig { path: p.clone(), bin_dir: p.clone() });
        }
    }
    None
}