//! Interface for users to interact with this application
//! Each command will query the server (via the client), and return the result
use clap::{Args, Parser, Subcommand};
use serde::Deserialize;
use std::{fmt::Write, fs::File, io::BufReader};

use crate::{
    client,
    errors::{EstimaterErrors, EstimaterResult},
};
use estimate_common::common::TaxInfo;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct EstimateCli {
    #[clap(subcommand)]
    command: EstimateCommands,
}

#[derive(Subcommand, Clone, Debug)]
enum EstimateCommands {
    /// Path to data file (in json format) representing what to process
    Config(FromConfigStruct),

    /// Manually input data via command line flags
    #[clap(name = "input")]
    CliArgs(TaxInfo),
}

#[derive(Args, Clone, Debug)]
struct FromConfigStruct {
    path_to_file: String,
}

impl FromConfigStruct {
    /// Validates the path
    ///
    /// # Return
    ///
    /// * The parsed config file as TaxInfo, or an error.
    fn validate_config_file(&self) -> EstimaterResult<TaxInfo> {
        let file = File::open(&self.path_to_file);
        match file {
            Err(err) => {
                let mut err_msg = format!(
                    "Incorrect path {} provided. File does not exist.",
                    &self.path_to_file
                );
                write!(err_msg, "\n{:?}", err)
                    .expect("Writting the error message for validating config failed");
                Err(EstimaterErrors::UserError(err_msg))
            }
            Ok(file) => {
                let file_reader = BufReader::new(file);
                Self::parse_config(file_reader)
            }
        }
    }

    /// Parses the config file and attempts to convert it to a known struct
    fn parse_config(file_reader: BufReader<File>) -> EstimaterResult<TaxInfo> {
        let mut de = serde_json::Deserializer::from_reader(file_reader);
        let contents = TaxInfo::deserialize(&mut de);
        match contents {
            Ok(tax_info) => Ok(tax_info),
            Err(err) => {
                let mut msg =
                    "The config file does not contain at LEAST one of the following: ".to_string();
                write!(
                    msg,
                    "gross_yearly_income, federal_tax_rate_percent, state_tax_rate_percent"
                )
                .unwrap();
                write!(msg, "\nError: {err}").unwrap();
                Err(EstimaterErrors::ParsingError(msg))
            }
        }
    }
}

impl EstimateCommands {
    /// Runs the commands after parsing
    pub fn run_command(cmd: EstimateCommands) -> EstimaterResult<()> {
        client::get_server_status();
        match cmd {
            EstimateCommands::Config(from_config_struct) => {
                // TODO - read in from a config file path'd
                println!(
                    "Reading from config file named {}",
                    from_config_struct.path_to_file
                );
                let tax_info: TaxInfo = from_config_struct.validate_config_file()?;
                println!("{}", tax_info);
                Ok(())
            }
            EstimateCommands::CliArgs(tax_info) => {
                println!("{}", tax_info);
                todo!()
            }
        }
    }
}
/// Entrance to the client by parsing CLI values and running commands
pub(crate) fn run_cli() {
    println!("Running cli!");

    let args = EstimateCli::parse();
    let cmd_res = EstimateCommands::run_command(args.command);

    match cmd_res {
        Err(err) => {
            println!("Error Running command : <print cmd>.\n Error: {}", err);
        }
        Ok(_res) => {
            println!("\n");
        }
    }
}
