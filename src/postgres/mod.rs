// src/postgres/mod.rs
pub mod commands;

use crate::config::DevEnvManager;
use std::process::Command;

pub fn init(manager: &DevEnvManager) {
    commands::run_with_config(manager, |cfg| {
        let initdb = cfg.bin_dir.join("initdb.exe");
        let data = cfg.path.join("data");
        let output = Command::new(&initdb)
            .args(&["-D", &data.to_string_lossy(), "-U", "postgres", "-E", "UTF8", "--locale=C", "-A", "trust"])
            .output()?;
        if output.status.success() {
            println!("Cluster inicializado: {}", data.display());
        } else {
            eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
        }
        Ok(())
    });
}

pub fn start(manager: &DevEnvManager) {
    commands::run_with_config(manager, |cfg| {
        let pg_ctl = cfg.bin_dir.join("pg_ctl.exe");
        let data = cfg.path.join("data");
        let log = cfg.path.join("logs").join("postgres.log");
        std::fs::create_dir_all(log.parent().unwrap())?;
        let output = Command::new(&pg_ctl)
            .args(&["start", "-D", &data.to_string_lossy(), "-l", &log.to_string_lossy()])
            .output()?;
        if output.status.success() {
            println!("Servidor corriendo: localhost:5432");
        } else {
            eprintln!("Error al iniciar");
        }
        Ok(())
    });
}

pub fn stop(manager: &DevEnvManager) {
    commands::run_with_config(manager, |cfg| {
        let pg_ctl = cfg.bin_dir.join("pg_ctl.exe");
        let data = cfg.path.join("data");
        let output = Command::new(&pg_ctl)
            .args(&["stop", "-D", &data.to_string_lossy(), "-m", "fast"])
            .output()?;
        if output.status.success() {
            println!("Servidor detenido");
        } else {
            eprintln!("Error al detener");
        }
        Ok(())
    });
}

pub fn restart(manager: &DevEnvManager) {
    commands::run_with_config(manager, |cfg| {
        let pg_ctl = cfg.bin_dir.join("pg_ctl.exe");
        let data = cfg.path.join("data");
        let log = cfg.path.join("logs").join("postgres.log");
        let output = Command::new(&pg_ctl)
            .args(&["restart", "-D", &data.to_string_lossy(), "-l", &log.to_string_lossy()])
            .output()?;
        if output.status.success() {
            println!("Servidor reiniciado");
        } else {
            eprintln!("Error al reiniciar");
        }
        Ok(())
    });
}

pub fn status(manager: &DevEnvManager) {
    commands::run_with_config(manager, |cfg| {
        let pg_ctl = cfg.bin_dir.join("pg_ctl.exe");
        let data = cfg.path.join("data");
        let output = Command::new(&pg_ctl)
            .args(&["status", "-D", &data.to_string_lossy()])
            .output()?;
        if output.status.success() {
            println!("Servidor corriendo");
            println!("{}", String::from_utf8_lossy(&output.stdout));
        } else {
            println!("Servidor detenido");
        }
        Ok(())
    });
}

pub fn list_dbs(manager: &DevEnvManager) {
    commands::run_with_config(manager, |cfg| {
        let psql = cfg.bin_dir.join("psql.exe");
        let output = Command::new(&psql)
            .args(&["-U", "postgres", "-l"])
            .output()?;
        println!("{}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    });
}

pub fn create_db(manager: &DevEnvManager, args: &[String]) {
    if args.len() < 3 { eprintln!("Falta nombre"); return; }
    commands::run_with_config(manager, |cfg| {
        let createdb = cfg.bin_dir.join("createdb.exe");
        let output = Command::new(&createdb)
            .args(&["-U", "postgres", &args[2]])
            .output()?;
        if output.status.success() {
            println!("Base de datos '{}' creada", &args[2]);
        } else {
            eprintln!("Error al crear");
        }
        Ok(())
    });
}

pub fn drop_db(manager: &DevEnvManager, args: &[String]) {
    if args.len() < 3 { 
        eprintln!("Falta nombre"); 
        return; 
    }
    let force = args.len() > 3 && args[3] == "--force";
    
    commands::run_with_config(manager, |cfg| {
        if !force {
            println!("¿Eliminar '{}'? Escribe 'SI':", &args[2]);
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).ok();  // ← .ok() evita propagar error
            if input.trim() != "SI" { 
                println!("Cancelado"); 
                return Ok(()); 
            }
        }
        
        let dropdb = cfg.bin_dir.join("dropdb.exe");
        let output = Command::new(&dropdb)
            .args(&["-U", "postgres", &args[2]])
            .output()?;  // ← ? correcto (output() → Result)
            
        if output.status.success() {
            println!("Base de datos '{}' eliminada", &args[2]);
        } else {
            eprintln!("Error al eliminar");
        }
        Ok(())
    });
}

pub fn backup_db(manager: &DevEnvManager, args: &[String]) {
    if args.len() < 4 { eprintln!("Uso: --backup-db <db> <file>"); return; }
    commands::run_with_config(manager, |cfg| {
        let pg_dump = cfg.bin_dir.join("pg_dump.exe");
        let output = Command::new(&pg_dump)
            .args(&["-U", "postgres", "-F", "c", "-f", &args[3], &args[2]])
            .output()?;
        if output.status.success() {
            println!("Respaldo: {}", &args[3]);
        } else {
            eprintln!("Error al respaldar");
        }
        Ok(())
    });
}

pub fn restore_db(manager: &DevEnvManager, args: &[String]) {
    if args.len() < 4 { eprintln!("Uso: --restore-db <db> <file>"); return; }
    commands::run_with_config(manager, |cfg| {
        let pg_restore = cfg.bin_dir.join("pg_restore.exe");
        let output = Command::new(&pg_restore)
            .args(&["-U", "postgres", "-d", &args[2], &args[3]])
            .output()?;
        if output.status.success() {
            println!("Base de datos restaurada");
        } else {
            eprintln!("Error al restaurar");
        }
        Ok(())
    });
}

pub fn psql(manager: &DevEnvManager, args: &[String]) {
    commands::run_with_config(manager, |cfg| {
        let mut cmd = Command::new(cfg.bin_dir.join("psql.exe"));
        cmd.arg("-U").arg("postgres");
        if args.len() > 2 { cmd.arg("-d").arg(&args[2]); }
        let _ = cmd.status()?;
        Ok(())
    });
}