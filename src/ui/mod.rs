// src/ui/mod.rs
use crate::config::DevEnvManager;

pub fn show_help() {
    println!("\nDevEnv CLI - Gestor de Entornos\n");
    println!("--setup-mingw, --setup-postgres, --setup-node");
    println!("--generate-script, --verify");
    println!("--init-postgres, --start-postgres, --psql, etc.");
    println!("--DQM27 → ayuda completa");
}

pub fn verify_tools(manager: &DevEnvManager) {
    println!("\nVerificación de Herramientas\n");
    if let Some(p) = manager.load_config(".mingw64-config") {
        println!("MinGW64: {}", p.display());
    } else {
        println!("MinGW64: No configurado");
    }
    if let Some(p) = manager.load_config(".postgres-config") {
        println!("PostgreSQL: {}", p.display());
    } else {
        println!("PostgreSQL: No configurado");
    }
    if let Some(p) = manager.load_config(".node-config") {
        println!("Node.js: {}", p.display());
    } else {
        println!("Node.js: No configurado");
    }
}