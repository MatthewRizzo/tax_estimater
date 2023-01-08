/// Implement a server to do the "hard" work relating to calculating the taxes.
use std::env;
use std::io;
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
    let tax_bracket =
        TaxBrackets::from_bracket_json(get_path_to_data("federal_tax_bracket.json").unwrap())?;
    let federal_tax = match tax_bracket.calculate_tax_amount(intermediate.taxable_income) {
        Err(err) => Err(EstimaterErrors::ServerError(format!(
            "Error calculating federal taxes: {err}"
        )))?,
        Ok(tax) => tax,
    };

    let state_tax = intermediate.taxable_income * (input_info.state_tax_rate_percent / 100.0);
    let net_income = (input_info.gross_yearly_income as f64)
        - -(federal_tax * intermediate.taxable_income)
        - (state_tax * intermediate.taxable_income);
    Ok(TaxResults::new(federal_tax, state_tax, net_income))
}

/// Represents data / results generated mid calculation that get reused.
struct IntermediateTaxData {
    taxable_income: f64,
}

impl IntermediateTaxData {
    pub(crate) fn new(input_info: &TaxInfo) -> Self {
        let taxable_income = input_info.gross_yearly_income as f64 - input_info.pre_tax_deducations;
        Self { taxable_income }
    }
}

/// Gets the path to a given data file relative to project root based on the cwd.
fn get_path_to_data(file_name: &str) -> io::Result<PathBuf> {
    let server_crate_path = env::current_dir()?;
    let project_root = server_crate_path
        .parent()
        .map_or_else(|| server_crate_path.as_path(), |root_path| root_path);
    let data_dir = project_root;
    let file_name = file_name.to_string();
    Ok(data_dir.join("data").join(file_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserializing() {
        // let path = "data/federal_tax_bracket.json";
        let path = get_path_to_data("federal_tax_bracket.json");
        assert!(path.is_ok(), "Path should be ok, but is {:?}", path.err());
        let data = TaxBrackets::from_bracket_json(path.unwrap());
        assert!(data.is_ok(), "data should be ok, but is {:?}", data.err())
    }

    #[test]
    fn test_calculate_taxes() {}
}
