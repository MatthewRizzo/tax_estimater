/// Implement a server to do the "hard" work relating to calculating the taxes.
use estimate_common::{
    common::{TaxInfo, TaxResults},
    errors::EstimaterResult,
};

pub fn calculate_taxes(input_info: TaxInfo) -> EstimaterResult<TaxResults> {
    let intermediate = IntermediateTaxData::new(&input_info);
    println!("taxable income {}", intermediate.taxable_income);
    let federal_tax = intermediate.taxable_income * (input_info.federal_tax_rate_percent / 100.0);
    let state_tax = intermediate.taxable_income * (input_info.state_tax_rate_percent / 100.0);
    let net_income = (input_info.gross_yearly_income as f32) -
        - (federal_tax * intermediate.taxable_income)
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
