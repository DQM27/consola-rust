// src/script/mod.rs
use crate::config::DevEnvManager;

pub fn generate(manager: &DevEnvManager) {
    let script_path = manager.config_dir.join("setup-env.bat");
    let mut script = String::from("@echo off\nREM Generado por DevEnv CLI\n\n");
    let mut has_config = false;

    if let Some(mingw) = manager.load_config(".mingw64-config") {
        has_config = true;
        let bin = mingw.join("bin");
        let inc = mingw.join("include");
        let lib = mingw.join("lib");
        script.push_str(&format!("set PATH={};%PATH%\n", bin.display()));
        script.push_str(&format!("set CC={}\\gcc.exe\n", bin.display()));
        script.push_str(&format!("set CXX={}\\g++.exe\n", bin.display()));
        script.push_str(&format!("set AR={}\\ar.exe\n", bin.display()));
        script.push_str(&format!("set C_INCLUDE_PATH={}\n", inc.display()));
        script.push_str(&format!("set LIBRARY_PATH={}\n", lib.display()));
        script.push_str("echo [OK] MinGW64\n\n");
    }

    if let Some(pg) = manager.load_config(".postgres-config") {
        has_config = true;
        let bin = pg.join("bin");
        let data = pg.join("data");
        let lib = pg.join("lib");
        script.push_str(&format!("set PATH={};%PATH%\n", bin.display()));
        script.push_str(&format!("set PGDATA={}\n", data.display()));
        script.push_str("set PGHOST=localhost\nset PGPORT=5432\nset PGUSER=postgres\n");
        script.push_str(&format!("set PQ_LIB_DIR={}\n", lib.display()));
        script.push_str(&format!("set PG_CONFIG={}\\pg_config.exe\n", bin.display()));
        script.push_str("echo [OK] PostgreSQL\n\n");
    }

    if let Some(node) = manager.load_config(".node-config") {
        has_config = true;
        script.push_str(&format!("set PATH={};%PATH%\n", node.display()));
        script.push_str("echo [OK] Node.js\n\n");
    }

    if !has_config {
        println!("No hay configuraciones");
        return;
    }

    script.push_str("echo.\necho [OK] Entorno configurado\n");
    std::fs::write(&script_path, script).unwrap();
    println!("Script generado: setup-env.bat");
}