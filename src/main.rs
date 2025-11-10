// devenv - Gestor de Entornos de Desarrollo
// Herramienta CLI para configurar Node.js, MinGW64 y PostgreSQL
// Modo comando exclusivo - No GUI

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, exit};

// ============================================
// ESTRUCTURAS DE DATOS
// ============================================

#[derive(Debug, Clone)]
struct EnvironmentConfig {
    path: PathBuf,
    bin_dir: PathBuf,
}

#[derive(Debug)]
struct DevEnvManager {
    config_dir: PathBuf,
}

// ============================================
// CONSTANTES
// ============================================

const CONFIG_FILE_MINGW: &str = ".mingw64-config";
const CONFIG_FILE_POSTGRES: &str = ".postgres-config";
const CONFIG_FILE_NODE: &str = ".node-config";
const SETUP_SCRIPT: &str = "setup-env.bat";
const SECRET_COMMAND: &str = "--DQM27";

// ============================================
// IMPLEMENTACIÓN DEL GESTOR
// ============================================

impl DevEnvManager {
    fn new() -> io::Result<Self> {
        let config_dir = env::current_dir()?;
        Ok(Self { config_dir })
    }

    fn load_config(&self, config_file: &str) -> Option<PathBuf> {
        let config_path = self.config_dir.join(config_file);
        if config_path.exists() {
            if let Ok(content) = fs::read_to_string(&config_path) {
                let path = PathBuf::from(content.trim());
                if path.exists() {
                    return Some(path);
                }
            }
        }
        None
    }

    fn save_config(&self, config_file: &str, path: &Path) -> io::Result<()> {
        let config_path = self.config_dir.join(config_file);
        fs::write(config_path, path.to_string_lossy().as_bytes())?;
        println!("✓ Configuración guardada: {}", config_file);
        Ok(())
    }

    fn find_mingw(&self) -> Option<EnvironmentConfig> {
        let search_paths = vec![
            self.config_dir.join("mingw64"),
            PathBuf::from("C:\\mingw64"),
            PathBuf::from("C:\\msys64\\mingw64"),
        ];

        for path in search_paths {
            let gcc = path.join("bin").join("gcc.exe");
            if gcc.exists() {
                return Some(EnvironmentConfig {
                    path: path.clone(),
                    bin_dir: path.join("bin"),
                });
            }
        }
        None
    }

    fn find_postgres(&self) -> Option<EnvironmentConfig> {
        let search_paths = vec![
            self.config_dir.clone(),
            self.config_dir.join("pgsql"),
            self.config_dir.join("postgresql"),
            PathBuf::from("C:\\pgsql"),
            PathBuf::from("C:\\Program Files\\PostgreSQL"),
        ];

        for path in search_paths {
            let postgres = path.join("bin").join("postgres.exe");
            if postgres.exists() {
                return Some(EnvironmentConfig {
                    path: path.clone(),
                    bin_dir: path.join("bin"),
                });
            }
        }
        None
    }

    fn find_node(&self) -> Option<EnvironmentConfig> {
        let search_paths = vec![
            PathBuf::from("C:\\Program Files\\nodejs"),
            PathBuf::from("C:\\Program Files (x86)\\nodejs"),
            self.config_dir.join("node"),
        ];

        for path in search_paths {
            let node = path.join("node.exe");
            if node.exists() {
                return Some(EnvironmentConfig {
                    path: path.clone(),
                    bin_dir: path.clone(),
                });
            }
        }
        None
    }

    fn request_manual_path(&self, tool_name: &str, validator: &str) -> Option<PathBuf> {
        print!("Ruta de {}: ", tool_name);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).ok()?;
        let path = PathBuf::from(input.trim().trim_matches('"'));

        if !path.exists() {
            println!("✗ Ruta no existe");
            return None;
        }

        let validator_path = path.join(validator);
        if !validator_path.exists() {
            println!("✗ {} no encontrado", validator);
            return None;
        }

        Some(path)
    }

    fn generate_setup_script(&self) -> io::Result<()> {
        let script_path = self.config_dir.join(SETUP_SCRIPT);
        let mut script = String::from("@echo off\n");
        script.push_str("REM Generado por DevEnv CLI\n\n");

        let mut has_config = false;

        if let Some(mingw) = self.load_config(CONFIG_FILE_MINGW) {
            has_config = true;
            let bin_dir = mingw.join("bin");
            let include_dir = mingw.join("include");
            let lib_dir = mingw.join("lib");
            
            script.push_str("REM MinGW64\n");
            script.push_str(&format!("set PATH={};%PATH%\n", bin_dir.display()));
            script.push_str(&format!("set CC={}\\gcc.exe\n", bin_dir.display()));
            script.push_str(&format!("set CXX={}\\g++.exe\n", bin_dir.display()));
            script.push_str(&format!("set AR={}\\ar.exe\n", bin_dir.display()));
            script.push_str(&format!("set C_INCLUDE_PATH={}\n", include_dir.display()));
            script.push_str(&format!("set LIBRARY_PATH={}\n", lib_dir.display()));
            script.push_str("echo [OK] MinGW64\n\n");
        }

        if let Some(pg) = self.load_config(CONFIG_FILE_POSTGRES) {
            has_config = true;
            let bin_dir = pg.join("bin");
            let data_dir = pg.join("data");
            let lib_dir = pg.join("lib");
            
            script.push_str("REM PostgreSQL\n");
            script.push_str(&format!("set PATH={};%PATH%\n", bin_dir.display()));
            script.push_str(&format!("set PGDATA={}\n", data_dir.display()));
            script.push_str("set PGHOST=localhost\n");
            script.push_str("set PGPORT=5432\n");
            script.push_str("set PGUSER=postgres\n");
            script.push_str(&format!("set PQ_LIB_DIR={}\n", lib_dir.display()));
            script.push_str(&format!("set PG_CONFIG={}\\pg_config.exe\n", bin_dir.display()));
            script.push_str("echo [OK] PostgreSQL\n\n");
        }

        if let Some(node) = self.load_config(CONFIG_FILE_NODE) {
            has_config = true;
            script.push_str("REM Node.js\n");
            script.push_str(&format!("set PATH={};%PATH%\n", node.display()));
            script.push_str("echo [OK] Node.js\n\n");
        }

        if !has_config {
            println!("✗ No hay configuraciones");
            return Ok(());
        }

        script.push_str("echo.\necho [OK] Entorno configurado\n");
        fs::write(&script_path, script)?;
        
        println!("✓ Script generado: {}", SETUP_SCRIPT);
        Ok(())
    }

    fn manage_postgres_server(&self, config: &EnvironmentConfig, action: &str) -> io::Result<()> {
        let pg_ctl = config.bin_dir.join("pg_ctl.exe");
        let data_dir = config.path.join("data");
        let log_file = config.path.join("logs").join("postgres.log");

        if let Some(parent) = log_file.parent() {
            fs::create_dir_all(parent)?;
        }

        if !data_dir.exists() && action != "init" {
            println!("✗ Cluster no inicializado");
            println!("  Ejecuta: devenv --init-postgres");
            return Ok(());
        }

        match action {
            "init" => {
                println!("🔧 Inicializando cluster...");
                let initdb = config.bin_dir.join("initdb.exe");
                let output = Command::new(&initdb)
                    .args(&["-D", &data_dir.to_string_lossy(), "-U", "postgres", "-E", "UTF8", "--locale=C", "-A", "trust"])
                    .output()?;
                
                if output.status.success() {
                    println!("✓ Cluster inicializado: {}", data_dir.display());
                } else {
                    println!("✗ Error al inicializar");
                    println!("{}", String::from_utf8_lossy(&output.stderr));
                }
            },
            "start" => {
                println!("🚀 Iniciando servidor...");
                let output = Command::new(&pg_ctl)
                    .args(&["start", "-D", &data_dir.to_string_lossy(), "-l", &log_file.to_string_lossy()])
                    .output()?;
                
                if output.status.success() {
                    println!("✓ Servidor corriendo: localhost:5432");
                } else {
                    println!("✗ Error al iniciar");
                }
            },
            "stop" => {
                println!("🛑 Deteniendo servidor...");
                let output = Command::new(&pg_ctl)
                    .args(&["stop", "-D", &data_dir.to_string_lossy(), "-m", "fast"])
                    .output()?;
                
                if output.status.success() {
                    println!("✓ Servidor detenido");
                } else {
                    println!("✗ Error al detener");
                }
            },
            "restart" => {
                println!("🔄 Reiniciando servidor...");
                let output = Command::new(&pg_ctl)
                    .args(&["restart", "-D", &data_dir.to_string_lossy(), "-l", &log_file.to_string_lossy()])
                    .output()?;
                
                if output.status.success() {
                    println!("✓ Servidor reiniciado");
                } else {
                    println!("✗ Error al reiniciar");
                }
            },
            "status" => {
                let output = Command::new(&pg_ctl)
                    .args(&["status", "-D", &data_dir.to_string_lossy()])
                    .output()?;
                
                if output.status.success() {
                    println!("✓ Servidor corriendo");
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                } else {
                    println!("✗ Servidor detenido");
                }
            },
            _ => println!("✗ Acción desconocida"),
        }

        Ok(())
    }

    fn list_databases(&self, config: &EnvironmentConfig) -> io::Result<()> {
        let psql = config.bin_dir.join("psql.exe");
        let output = Command::new(&psql)
            .args(&["-U", "postgres", "-l"])
            .output()?;

        if output.status.success() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        } else {
            println!("✗ Error al listar bases de datos");
        }

        Ok(())
    }

    fn create_database(&self, config: &EnvironmentConfig, db_name: &str) -> io::Result<()> {
        println!("🔨 Creando base de datos '{}'...", db_name);
        
        let createdb = config.bin_dir.join("createdb.exe");
        let output = Command::new(&createdb)
            .args(&["-U", "postgres", db_name])
            .output()?;

        if output.status.success() {
            println!("✓ Base de datos '{}' creada", db_name);
        } else {
            println!("✗ Error al crear base de datos");
        }

        Ok(())
    }

    fn drop_database(&self, config: &EnvironmentConfig, db_name: &str, force: bool) -> io::Result<()> {
        if !force {
            println!("⚠️  ¿Eliminar '{}'? (Esta acción no se puede deshacer)", db_name);
            print!("Escribe 'SI' para confirmar: ");
            io::stdout().flush()?;

            let mut confirmation = String::new();
            io::stdin().read_line(&mut confirmation)?;

            if confirmation.trim() != "SI" {
                println!("✗ Cancelado");
                return Ok(());
            }
        }

        let dropdb = config.bin_dir.join("dropdb.exe");
        let output = Command::new(&dropdb)
            .args(&["-U", "postgres", db_name])
            .output()?;

        if output.status.success() {
            println!("✓ Base de datos '{}' eliminada", db_name);
        } else {
            println!("✗ Error al eliminar");
        }

        Ok(())
    }

    fn backup_database(&self, config: &EnvironmentConfig, db_name: &str, output_file: &str) -> io::Result<()> {
        println!("💾 Respaldando '{}'...", db_name);
        
        let pg_dump = config.bin_dir.join("pg_dump.exe");
        let output = Command::new(&pg_dump)
            .args(&["-U", "postgres", "-F", "c", "-f", output_file, db_name])
            .output()?;

        if output.status.success() {
            println!("✓ Respaldo: {}", output_file);
        } else {
            println!("✗ Error al respaldar");
        }

        Ok(())
    }

    fn restore_database(&self, config: &EnvironmentConfig, db_name: &str, backup_file: &str) -> io::Result<()> {
        println!("📥 Restaurando '{}'...", db_name);
        
        let pg_restore = config.bin_dir.join("pg_restore.exe");
        let output = Command::new(&pg_restore)
            .args(&["-U", "postgres", "-d", db_name, backup_file])
            .output()?;

        if output.status.success() {
            println!("✓ Base de datos restaurada");
        } else {
            println!("✗ Error al restaurar");
        }

        Ok(())
    }

    fn connect_psql(&self, config: &EnvironmentConfig, db_name: Option<&str>) -> io::Result<()> {
        let psql = config.bin_dir.join("psql.exe");
        
        let mut cmd = Command::new(&psql);
        cmd.arg("-U").arg("postgres");
        
        if let Some(db) = db_name {
            cmd.arg("-d").arg(db);
        }

        let status = cmd.status()?;
        if !status.success() {
            println!("✗ Error al conectar");
        }

        Ok(())
    }

    fn verify_tools(&self) {
        println!("\n╔═══════════════════════════════════════════════════╗");
        println!("║            Verificación de Herramientas          ║");
        println!("╚═══════════════════════════════════════════════════╝\n");

        if let Some(config) = self.load_config(CONFIG_FILE_MINGW) {
            let gcc = config.join("bin").join("gcc.exe");
            if gcc.exists() {
                if let Ok(output) = Command::new(&gcc).arg("--version").output() {
                    let version = String::from_utf8_lossy(&output.stdout);
                    let first_line = version.lines().next().unwrap_or("Desconocido");
                    println!("✓ MinGW64: {}", first_line);
                    println!("  Ruta: {}", config.display());
                }
            }
        } else {
            println!("✗ MinGW64: No configurado");
        }

        println!();

        if let Some(config) = self.load_config(CONFIG_FILE_POSTGRES) {
            let postgres = config.join("bin").join("postgres.exe");
            if postgres.exists() {
                if let Ok(output) = Command::new(&postgres).arg("--version").output() {
                    let version = String::from_utf8_lossy(&output.stdout);
                    println!("✓ PostgreSQL: {}", version.trim());
                    println!("  Ruta: {}", config.display());
                }
            }
        } else {
            println!("✗ PostgreSQL: No configurado");
        }

        println!();

        if let Some(config) = self.load_config(CONFIG_FILE_NODE) {
            let node = config.join("node.exe");
            if node.exists() {
                if let Ok(output) = Command::new(&node).arg("--version").output() {
                    let version = String::from_utf8_lossy(&output.stdout);
                    println!("✓ Node.js: {}", version.trim());
                    println!("  Ruta: {}", config.display());
                }
            }
        } else {
            println!("✗ Node.js: No configurado");
        }

        println!();
    }
}

// ============================================
// AYUDA Y COMANDOS
// ============================================

fn show_help() {
    println!("\n╔═══════════════════════════════════════════════════╗");
    println!("║          DevEnv CLI - Gestor de Entornos            ║");
    println!("╚═══════════════════════════════════════════════════╝\n");
    
    println!("CONFIGURACIÓN:");
    println!("  --setup-mingw          Buscar y configurar MinGW64");
    println!("  --setup-postgres       Buscar y configurar PostgreSQL");
    println!("  --setup-node           Buscar y configurar Node.js");
    println!("  --generate-script      Generar setup-env.bat");
    println!("  --verify               Verificar herramientas instaladas");
    
    println!("\nPOSTGRESQL - SERVIDOR:");
    println!("  --init-postgres        Inicializar cluster (primera vez)");
    println!("  --start-postgres       Iniciar servidor");
    println!("  --stop-postgres        Detener servidor");
    println!("  --restart-postgres     Reiniciar servidor");
    println!("  --status-postgres      Ver estado del servidor");
    
    println!("\nPOSTGRESQL - BASES DE DATOS:");
    println!("  --list-dbs             Listar todas las bases de datos");
    println!("  --create-db <nombre>   Crear base de datos");
    println!("  --drop-db <nombre>     Eliminar base de datos");
    println!("  --backup-db <db> <archivo.dump>");
    println!("                         Respaldar base de datos");
    println!("  --restore-db <db> <archivo.dump>");
    println!("                         Restaurar base de datos");
    println!("  --psql [db]            Conectar con psql (opcional: nombre db)");
    
    println!("\nOTROS:");
    println!("  --DQM27                Mostrar esta ayuda");
    
    println!("\nEJEMPLOS:");
    println!("  devenv --setup-postgres");
    println!("  devenv --start-postgres");
    println!("  devenv --create-db mi_base");
    println!("  devenv --backup-db mi_base backup.dump");
    println!("  devenv --psql mi_base");
    println!();
}

// ============================================
// MAIN
// ============================================

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    // Si no hay argumentos, salir silenciosamente
    if args.len() < 2 {
        exit(0);
    }

    let manager = DevEnvManager::new()?;
    let command = args[1].as_str();

    match command {
        SECRET_COMMAND => {
            show_help();
        },
        "--setup-mingw" => {
            println!("🔍 Configurando MinGW64...");
            let config = if let Some(saved) = manager.load_config(CONFIG_FILE_MINGW) {
                println!("✓ Configuración existente: {}", saved.display());
                Some(EnvironmentConfig {
                    path: saved.clone(),
                    bin_dir: saved.join("bin"),
                })
            } else {
                manager.find_mingw().or_else(|| {
                    println!("No encontrado automáticamente");
                    manager.request_manual_path("MinGW64", "bin\\gcc.exe")
                        .map(|path| EnvironmentConfig {
                            path: path.clone(),
                            bin_dir: path.join("bin"),
                        })
                })
            };

            if let Some(cfg) = config {
                manager.save_config(CONFIG_FILE_MINGW, &cfg.path)?;
                println!("💡 Ejecuta: devenv --generate-script");
            } else {
                println!("✗ No se pudo configurar MinGW64");
            }
        },
        "--setup-postgres" => {
            println!("🔍 Configurando PostgreSQL...");
            let config = if let Some(saved) = manager.load_config(CONFIG_FILE_POSTGRES) {
                println!("✓ Configuración existente: {}", saved.display());
                Some(EnvironmentConfig {
                    path: saved.clone(),
                    bin_dir: saved.join("bin"),
                })
            } else {
                manager.find_postgres().or_else(|| {
                    println!("No encontrado automáticamente");
                    manager.request_manual_path("PostgreSQL", "bin\\postgres.exe")
                        .map(|path| EnvironmentConfig {
                            path: path.clone(),
                            bin_dir: path.join("bin"),
                        })
                })
            };

            if let Some(cfg) = config {
                manager.save_config(CONFIG_FILE_POSTGRES, &cfg.path)?;
                println!("💡 Ejecuta: devenv --init-postgres");
            } else {
                println!("✗ No se pudo configurar PostgreSQL");
            }
        },
        "--setup-node" => {
            println!("🔍 Configurando Node.js...");
            let config = if let Some(saved) = manager.load_config(CONFIG_FILE_NODE) {
                println!("✓ Configuración existente: {}", saved.display());
                Some(EnvironmentConfig {
                    path: saved.clone(),
                    bin_dir: saved.clone(),
                })
            } else {
                manager.find_node().or_else(|| {
                    println!("No encontrado automáticamente");
                    manager.request_manual_path("Node.js", "node.exe")
                        .map(|path| EnvironmentConfig {
                            path: path.clone(),
                            bin_dir: path.clone(),
                        })
                })
            };

            if let Some(cfg) = config {
                manager.save_config(CONFIG_FILE_NODE, &cfg.path)?;
                println!("💡 Ejecuta: devenv --generate-script");
            } else {
                println!("✗ No se pudo configurar Node.js");
            }
        },
        "--generate-script" => {
            manager.generate_setup_script()?;
        },
        "--verify" => {
            manager.verify_tools();
        },
        "--init-postgres" => {
            if let Some(pg_path) = manager.load_config(CONFIG_FILE_POSTGRES) {
                let config = EnvironmentConfig {
                    path: pg_path.clone(),
                    bin_dir: pg_path.join("bin"),
                };
                manager.manage_postgres_server(&config, "init")?;
            } else {
                println!("✗ PostgreSQL no configurado");
                println!("  Ejecuta: devenv --setup-postgres");
            }
        },
        "--start-postgres" => {
            if let Some(pg_path) = manager.load_config(CONFIG_FILE_POSTGRES) {
                let config = EnvironmentConfig {
                    path: pg_path.clone(),
                    bin_dir: pg_path.join("bin"),
                };
                manager.manage_postgres_server(&config, "start")?;
            } else {
                println!("✗ PostgreSQL no configurado");
            }
        },
        "--stop-postgres" => {
            if let Some(pg_path) = manager.load_config(CONFIG_FILE_POSTGRES) {
                let config = EnvironmentConfig {
                    path: pg_path.clone(),
                    bin_dir: pg_path.join("bin"),
                };
                manager.manage_postgres_server(&config, "stop")?;
            } else {
                println!("✗ PostgreSQL no configurado");
            }
        },
        "--restart-postgres" => {
            if let Some(pg_path) = manager.load_config(CONFIG_FILE_POSTGRES) {
                let config = EnvironmentConfig {
                    path: pg_path.clone(),
                    bin_dir: pg_path.join("bin"),
                };
                manager.manage_postgres_server(&config, "restart")?;
            } else {
                println!("✗ PostgreSQL no configurado");
            }
        },
        "--status-postgres" => {
            if let Some(pg_path) = manager.load_config(CONFIG_FILE_POSTGRES) {
                let config = EnvironmentConfig {
                    path: pg_path.clone(),
                    bin_dir: pg_path.join("bin"),
                };
                manager.manage_postgres_server(&config, "status")?;
            } else {
                println!("✗ PostgreSQL no configurado");
            }
        },
        "--list-dbs" => {
            if let Some(pg_path) = manager.load_config(CONFIG_FILE_POSTGRES) {
                let config = EnvironmentConfig {
                    path: pg_path.clone(),
                    bin_dir: pg_path.join("bin"),
                };
                manager.list_databases(&config)?;
            } else {
                println!("✗ PostgreSQL no configurado");
            }
        },
        "--create-db" => {
            if args.len() < 3 {
                println!("✗ Falta nombre de base de datos");
                println!("  Uso: devenv --create-db <nombre>");
                exit(1);
            }
            
            if let Some(pg_path) = manager.load_config(CONFIG_FILE_POSTGRES) {
                let config = EnvironmentConfig {
                    path: pg_path.clone(),
                    bin_dir: pg_path.join("bin"),
                };
                manager.create_database(&config, &args[2])?;
            } else {
                println!("✗ PostgreSQL no configurado");
            }
        },
        "--drop-db" => {
            if args.len() < 3 {
                println!("✗ Falta nombre de base de datos");
                exit(1);
            }
            
            let force = args.len() > 3 && args[3] == "--force";
            
            if let Some(pg_path) = manager.load_config(CONFIG_FILE_POSTGRES) {
                let config = EnvironmentConfig {
                    path: pg_path.clone(),
                    bin_dir: pg_path.join("bin"),
                };
                manager.drop_database(&config, &args[2], force)?;
            } else {
                println!("✗ PostgreSQL no configurado");
            }
        },
        "--backup-db" => {
            if args.len() < 4 {
                println!("✗ Faltan parámetros");
                println!("  Uso: devenv --backup-db <db> <archivo.dump>");
                exit(1);
            }
            
            if let Some(pg_path) = manager.load_config(CONFIG_FILE_POSTGRES) {
                let config = EnvironmentConfig {
                    path: pg_path.clone(),
                    bin_dir: pg_path.join("bin"),
                };
                manager.backup_database(&config, &args[2], &args[3])?;
            } else {
                println!("✗ PostgreSQL no configurado");
            }
        },
        "--restore-db" => {
            if args.len() < 4 {
                println!("✗ Faltan parámetros");
                println!("  Uso: devenv --restore-db <db> <archivo.dump>");
                exit(1);
            }
            
            if let Some(pg_path) = manager.load_config(CONFIG_FILE_POSTGRES) {
                let config = EnvironmentConfig {
                    path: pg_path.clone(),
                    bin_dir: pg_path.join("bin"),
                };
                manager.restore_database(&config, &args[2], &args[3])?;
            } else {
                println!("✗ PostgreSQL no configurado");
            }
        },
        "--psql" => {
            if let Some(pg_path) = manager.load_config(CONFIG_FILE_POSTGRES) {
                let config = EnvironmentConfig {
                    path: pg_path.clone(),
                    bin_dir: pg_path.join("bin"),
                };
                
                let db_name = if args.len() > 2 {
                    Some(args[2].as_str())
                } else {
                    None
                };
                
                manager.connect_psql(&config, db_name)?;
            } else {
                println!("✗ PostgreSQL no configurado");
            }
        },
        _ => {
            // Comando desconocido, no hacer nada (silencio total)
            exit(0);
        }
    }

    Ok(())
}