// src/config/mod.rs
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Write};

#[derive(Debug)]
pub struct DevEnvManager {
    pub config_dir: PathBuf,
}

impl DevEnvManager {
    pub fn new() -> io::Result<Self> {
        Ok(Self { config_dir: std::env::current_dir()? })
    }

    pub fn load_config(&self, file: &str) -> Option<PathBuf> {
        let path = self.config_dir.join(file);
        fs::read_to_string(&path).ok().and_then(|s| {
            let p = PathBuf::from(s.trim());
            p.exists().then_some(p)
        })
    }

    pub fn save_config(&self, file: &str, path: &Path) -> io::Result<()> {
        fs::write(self.config_dir.join(file), path.to_string_lossy().as_bytes())?;
        println!("Configuración guardada: {}", file);
        Ok(())
    }

    pub fn request_manual_path(&self, name: &str, validator: &str) -> Option<PathBuf> {
        print!("Ruta de {}: ", name);
        let _ = io::stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok()?;
        let path = PathBuf::from(input.trim().trim_matches('"'));
        if path.exists() && path.join(validator).exists() {
            Some(path)
        } else {
            println!("Ruta inválida");
            None
        }
    }
} // ← ESTE `}` ES EL CIERRE DEL `impl DevEnvManager`