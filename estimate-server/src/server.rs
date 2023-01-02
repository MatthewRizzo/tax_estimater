/// Implement a server to do the "hard" work relating to calculating the taxes.
use serde_json;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::PathBuf;

use estimate_common::{
    common::{TaxInfo, TaxResults},
    errors::{EstimaterErrors, EstimaterResult},
};

use crate::tax_bracket::TaxBrackets;

pub fn calculate_taxes(input_info: TaxInfo) -> EstimaterResult<TaxResults> {
    let intermediate = IntermediateTaxData::new(&input_info);
    println!("taxable income {}", intermediate.taxable_income);

    // TODO: Add path to json file as part of Client CLI input / what is passed to server
    let tax_brackets = read_from_bracket_json(get_path_to_data().unwrap())?;
    print!("{}", tax_brackets);

    let federal_tax = intermediate.taxable_income * (input_info.federal_tax_rate_percent / 100.0);
    let state_tax = intermediate.taxable_income * (input_info.state_tax_rate_percent / 100.0);
    let net_income = (input_info.gross_yearly_income as f32)
        - -(federal_tax * intermediate.taxable_income)
        - (state_tax * intermediate.taxable_income);
    Ok(TaxResults::new(federal_tax, state_tax, net_income))
}

/// Represents data / results generated mid calculation that get reused.
struct IntermediateTaxData {
    taxable_income: f32,
}

impl IntermediateTaxData {
    pub(crate) fn new(input_info: &TaxInfo) -> Self {
        let taxable_income = input_info.gross_yearly_income as f32 - input_info.pre_tax_deducations;
        Self { taxable_income }
    }
}

fn get_path_to_data() -> io::Result<PathBuf> {
    let server_crate_path = env::current_dir()?;
    let project_root = server_crate_path
        .parent()
        .map_or_else(|| server_crate_path.as_path(), |root_path| root_path);
    let data_dir = project_root;
    let file_name = "federal_tax_bracket.json".to_string();
    Ok(data_dir.join("data").join(file_name))
}

/// Attempts to read from the json containing tax bracket info.
///
/// # Return
///
/// * Error if file doesn't exist (or something else)
/// * Success: The read in json value
fn read_from_bracket_json(path: PathBuf) -> EstimaterResult<TaxBrackets> {
    let file = File::open(&path);
    if let Ok(opened_file) = file {
        let read_buffer = BufReader::new(opened_file);
        let mut brackets: TaxBrackets =
            serde_json::from_reader(read_buffer).map_err(EstimaterErrors::SerdeDeserializeError)?;
        brackets.sort_brackets();
        brackets.tabulate_cumulative_taxes();
        brackets.validate_all_brackets()?;
        Ok(brackets)
    } else {
        Err(EstimaterErrors::FileError(format!(
            "The file {:?} does not exist",
            path
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserializing() {
        // let path = "data/federal_tax_bracket.json";
        let path = get_path_to_data();
        assert!(path.is_ok(), "Path should be ok, but is {:?}", path.err());
        let data = read_from_bracket_json(path.unwrap());
        assert!(data.is_ok(), "data should be ok, but is {:?}", data.err())
    }
}
