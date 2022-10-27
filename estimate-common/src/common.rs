use clap::Args;
use serde::Deserialize;
use std::fmt;

#[derive(Args, Clone, Debug, Deserialize)]
pub struct TaxInfo {
    pub gross_yearly_income: u64,
    pub federal_tax_rate_percent: f32,
    pub state_tax_rate_percent: f32,
}

impl fmt::Display for TaxInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Tax info: gross income: {}, state tax: {}, federal tax: {}",
            self.gross_yearly_income, self.state_tax_rate_percent, self.federal_tax_rate_percent
        )
    }
}
