mod app_builder;

use std::io::{self};
use app_builder::run_app;
use v030_domain::configuration;
use configuration::Configuration;
use clap::Parser;

fn main() -> io::Result<()> {
    let initial_configuration = Configuration::parse(); // Obtenir la configuration initiale

    // Appeler la fonction run_app avec la configuration initiale
    run_app(initial_configuration);

    Ok(())
}