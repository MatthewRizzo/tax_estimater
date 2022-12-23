use clap::Args;
use serde::Deserialize;
use std::fmt;

#[derive(Args, Clone, Debug, Deserialize)]
pub struct TaxInfo {
    #[clap(long = "gross")]
    pub gross_yearly_income: u64,
    #[clap(long = "federal")]
    pub federal_tax_rate_percent: f32,
    #[clap(long = "state")]
    pub state_tax_rate_percent: f32,
    #[clap(short, long = "pre-tax-deductions")]
    pub pre_tax_deducations: f32,
}

impl fmt::Display for TaxInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Tax info: gross income: {} (deducations = {}), state tax: {}, federal tax: {}",
            self.gross_yearly_income,
            self.pre_tax_deducations,
            self.state_tax_rate_percent,
            self.federal_tax_rate_percent
        )
    }
}
