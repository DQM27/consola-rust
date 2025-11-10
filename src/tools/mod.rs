// src/tools/mod.rs
pub mod mingw;
pub mod postgres;
pub mod node;

use crate::config::DevEnvManager;

pub fn setup_mingw(manager: &DevEnvManager, _args: &[String]) {
    mingw::setup(manager);
}

pub fn setup_postgres(manager: &DevEnvManager, _args: &[String]) {
    postgres::setup(manager);
}

pub fn setup_node(manager: &DevEnvManager, _args: &[String]) {
    node::setup(manager);
}