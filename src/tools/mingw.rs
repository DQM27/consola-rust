// src/tools/mingw.rs
use crate::config::DevEnvManager;
use crate::path;
use std::path::PathBuf;

#[derive(Clone)]
pub struct EnvironmentConfig {
    pub path: PathBuf,
    pub bin_dir: PathBuf,
}

pub fn setup(manager: &DevEnvManager) {
    println!("Configurando MinGW64...");
    let config = if let Some(saved) = manager.load_config(".mingw64-config") {
        println!("Configuración existente: {}", saved.display());
        Some(EnvironmentConfig {
            path: saved.clone(),
            bin_dir: saved.join("bin"),
        })
    } else {
        find_mingw(manager).or_else(|| {
            println!("No encontrado automáticamente");
            manager.request_manual_path("MinGW64", "bin\\gcc.exe")
                .map(|p| EnvironmentConfig { path: p.clone(), bin_dir: p.join("bin") })
        })
    };

    if let Some(cfg) = config {
        let _ = manager.save_config(".mingw64-config", &cfg.path);
        let _ = path::add_to_user_path(&cfg.bin_dir);
        println!("Ejecuta: devenv --generate-script");
    } else {
        println!("No se pudo configurar MinGW64");
    }
}

fn find_mingw(manager: &DevEnvManager) -> Option<EnvironmentConfig> {
    let paths = vec![
        manager.config_dir.join("mingw64"),
        "C:\\mingw64".into(),
        "C:\\msys64\\mingw64".into(),
    ];
    for p in paths {
        let exe = p.join("bin").join("gcc.exe");
        if exe.exists() {
            return Some(EnvironmentConfig { path: p.clone(), bin_dir: p.join("bin") });
        }
    }
    None
}