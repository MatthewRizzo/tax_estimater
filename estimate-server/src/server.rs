/// Implement a server to do the "hard" work relating to calculating the taxes.
use std::env;
use std::io;
use std::path::PathBuf;

use estimate_common::{
    common::{TaxInfo, TaxResults},
    errors::{EstimaterErrors, EstimaterResult},
};

use crate::tax_bracket::TaxBrackets;

/// Calculates the taxes that will be levied for the given input
///
/// # Return
///
/// * `Error`: Some error explaining why the calculation could not be completed
/// * `Ok(TaxResults)`: A breakdown of the taxes paid and the net income result
pub fn calculate_taxes(input_info: &TaxInfo) -> EstimaterResult<TaxResults> {
    let intermediate = IntermediateTaxData::new(input_info);

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
    let net_income = (input_info.gross_yearly_income as f64) - federal_tax - state_tax;
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
    fn test_calculate_taxes() {
        // TODO: remove federal and state tax % once the API is updated to relfect the change in
        // server implementation.
        let test_input_info = TaxInfo {
            gross_yearly_income: 50000,
            federal_tax_rate_percent: 0.0,
            state_tax_rate_percent: 5.0,
            pre_tax_deducations: 0.0,
        };

        let calculate_res =
            calculate_taxes(&test_input_info).expect("Tax calculation should've worked");
        assert!(
            calculate_res.state_tax == 2500.0,
            "Expected: 2500.0. Got: {}",
            calculate_res.state_tax
        );
        assert!(
            calculate_res.federal_tax == 6617.0,
            "Income {}. Federal Tax Expected: 6617.0. Got: {}",
            50000,
            calculate_res.federal_tax
        );

        let test_input_info2 = TaxInfo {
            gross_yearly_income: 100000,
            federal_tax_rate_percent: 0.0,
            state_tax_rate_percent: 5.0,
            pre_tax_deducations: 0.0,
        };
        let calculate_res =
            calculate_taxes(&test_input_info2).expect("Tax calculation should've worked");
        assert!(
            calculate_res.federal_tax == 17835.5,
            "Income: {}. Federal Tax Expected: 17835.5. Got: {}",
            100000,
            calculate_res.federal_tax
        );
    }
}
