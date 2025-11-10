// src/path/mod.rs
use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;

pub fn add_to_user_path(bin_path: &Path) -> std::io::Result<()> {
    let path_str = bin_path.to_string_lossy().to_string();
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu.create_subkey(r"Environment")?;
    let current: String = env.get_value("Path").unwrap_or_default();

    if current.split(';').any(|p| p.eq_ignore_ascii_case(&path_str)) {
        println!("Ya en PATH: {}", path_str);
        return Ok(());
    }

    let new_path = if current.is_empty() || current.ends_with(';') {
        format!("{}{}", current, path_str)
    } else {
        format!("{};{}", current, path_str)
    };

    env.set_value("Path", &new_path)?;
    println!("Añadido al PATH de usuario: {}", path_str);
    println!("Cierra y abre terminal para aplicar");
    Ok(())
}