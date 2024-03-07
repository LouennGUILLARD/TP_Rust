mod app_builder;

use std::io::{self, BufRead};
use std::collections::BTreeMap as Map;
use app_builder::run_app;
use v021_app_builder::configuration;
use configuration::Configuration;
use clap::Parser;

fn main() -> io::Result<()> {
    let initial_configuration = Configuration::parse(); // Obtenir la configuration initiale

    // Appeler la fonction run_app avec la configuration initiale
    run_app(initial_configuration);

    Ok(())
}