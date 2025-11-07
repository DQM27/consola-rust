# ============================================
# Rust + MinGW64 PORTÁTIL (SIN POSTGRESQL)
# Para PCs con políticas restrictivas
# ============================================
param(
    [switch]$Verify,
    [switch]$Reconfigure,
    [switch]$Clean
)

Clear-Host
Write-Host "Rust + MinGW64 (Portable)" -ForegroundColor Magenta
Write-Host "══════════════════════════════" -ForegroundColor Magenta
Write-Host ""

# ============================================
# 1. RUTAS LOCALES (todo en esta carpeta)
# ============================================
$BaseDir     = $PSScriptRoot
$MingwDir    = Join-Path $BaseDir "mingw64"
$ConfigFile  = Join-Path $BaseDir ".mingw64-config"

# ============================================
# 2. LIMPIAR CONFIGURACIÓN
# ============================================
if ($Clean) {
    if (Test-Path $ConfigFile) { Remove-Item $ConfigFile -Force }
    Write-Host "[OK] Configuración limpiada" -ForegroundColor Green
    exit 0
}

# ============================================
# 3. BUSCAR MINGW64 LOCAL
# ============================================
function Find-LocalMinGW {
    $candidates = @(
        $MingwDir,
        (Join-Path $BaseDir "MinGW64"),
        (Join-Path $BaseDir "tools\mingw64"),
        (Join-Path (Split-Path $BaseDir) "mingw64")
    )
    foreach ($path in $candidates) {
        if (Test-Path "$path\bin\gcc.exe") {
            Write-Host "[OK] MinGW64 encontrado: $path" -ForegroundColor Green
            return $path
        }
    }
    return $null
}

# ============================================
# 4. CARGAR O CONFIGURAR MINGW64
# ============================================
$mingwPath = $null

# 1. Cargar desde archivo de config
if (Test-Path $ConfigFile) {
    $saved = (Get-Content $ConfigFile -Raw).Trim()
    if (Test-Path "$saved\bin\gcc.exe") {
        $mingwPath = $saved
        Write-Host "[OK] MinGW64 cargado desde .mingw64-config" -ForegroundColor Green
    } else {
        Remove-Item $ConfigFile -Force
    }
}

# 2. Buscar en carpeta local
if (-not $mingwPath) {
    $mingwPath = Find-LocalMinGW
}

# 3. Reconfigurar si se pide o no hay
if ($Reconfigure -or -not $mingwPath) {
    Write-Host ""
    Write-Host "MinGW64 no encontrado" -ForegroundColor Yellow
    Write-Host "Coloca la carpeta 'mingw64' junto a este script" -ForegroundColor Cyan
    Write-Host "Descarga portable desde:" -ForegroundColor White
    Write-Host "https://github.com/niXman/mingw-builds-binaries/releases" -ForegroundColor Gray
    Write-Host "Recomendado: x86_64-15.2.0-release-posix-seh-ucrt-rt_v13-rev0.7z" -ForegroundColor Gray
    Write-Host ""
    $confirm = Read-Host "Ruta manual de MinGW64 (o Enter para cancelar)"
    if ($confirm) {
        $confirm = $confirm.Trim('"').Trim()
        if (Test-Path "$confirm\bin\gcc.exe") {
            $mingwPath = $confirm
        } else {
            Write-Host "[ERROR] gcc.exe no encontrado en esa ruta" -ForegroundColor Red
            exit 1
        }
    } else {
        Write-Host "[ERROR] MinGW64 es obligatorio" -ForegroundColor Red
        exit 1
    }
}

# Guardar configuración
$mingwPath | Out-File $ConfigFile -Encoding UTF8 -Force

# ============================================
# 5. CONFIGURAR ENTORNO (solo esta sesión)
# ============================================
$mingwBin = Join-Path $mingwPath "bin"

# Agregar al PATH de esta sesión
$env:PATH = "$mingwBin;$env:USERPROFILE\.cargo\bin;$env:PATH"

# Variables para Rust + C/C++
$env:CC  = "$mingwBin\gcc.exe"
$env:CXX = "$mingwBin\g++.exe"
$env:AR  = "$mingwBin\ar.exe"
$env:C_INCLUDE_PATH     = Join-Path $mingwPath "include"
$env:CPLUS_INCLUDE_PATH = Join-Path $mingwPath "include"
$env:LIBRARY_PATH       = Join-Path $mingwPath "lib"

Write-Host "[OK] Entorno configurado para esta sesión" -ForegroundColor Green
Write-Host ""

# ============================================
# 6. VERIFICAR (opcional)
# ============================================
if ($Verify) {
    Write-Host "Verificación del entorno:" -ForegroundColor Cyan
    $cmds = @(
        "gcc --version",
        "g++ --version",
        "rustc --version",
        "cargo --version"
    )
    foreach ($cmd in $cmds) {
        try {
            $out = & $cmd.Split(" ")[0] $cmd.Split(" ", 2)[1] 2>$null | Select-Object -First 1
            Write-Host " $cmd → $out" -ForegroundColor Green
        } catch {
            Write-Host " $cmd → [ERROR]" -ForegroundColor Red
        }
    }
    Write-Host ""
    exit 0
}

# ============================================
# 7. RESUMEN FINAL
# ============================================
Write-Host "Listo! Usa:" -ForegroundColor Green
Write-Host ""
Write-Host "   cargo build" -ForegroundColor White
Write-Host "   cargo run"   -ForegroundColor White
Write-Host ""
Write-Host "Este script solo afecta esta ventana." -ForegroundColor Gray
Write-Host "Ejecútalo cada vez que abras PowerShell." -ForegroundColor Gray
Write-Host ""

# Auto-compilar si hay proyecto
if (Test-Path "Cargo.toml") {
    $name = (Get-Content "Cargo.toml" | Select-String 'name = "(.+)"').Matches.Groups[1].Value
    Write-Host "Proyecto: $name" -ForegroundColor Cyan
    $run = Read-Host "Ejecutar 'cargo build'? (s/N)"
    if ($run -match "^[sS]") {
        cargo build
    }
}