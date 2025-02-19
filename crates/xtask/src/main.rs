mod commands;

use clap::{AppSettings, Parser as _};

use crate::commands::generate_c_header::ArgsGenerateCHeader;

#[derive(clap::Parser)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
enum Args {
    /// Generate voicevox_core.h
    GenerateCHeader(ArgsGenerateCHeader),
}

fn main() -> eyre::Result<()> {
    let args = Args::parse();
    color_eyre::install()?;
    match args {
        Args::GenerateCHeader(args) => commands::generate_c_header::run(args),
    }
}
