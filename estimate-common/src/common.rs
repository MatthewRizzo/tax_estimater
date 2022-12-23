use clap::Args;
use serde::Deserialize;
use std::fmt;

#[derive(Args, Clone, Debug, Deserialize)]
pub struct TaxInfo {
    #[clap(long = "gross")]
    pub gross_yearly_income: u64,
    #[clap(long = "federal")]
    /// Federal tax as a %
    pub federal_tax_rate_percent: f32,
    #[clap(long = "state")]
    /// State tax as a %
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

/// Struct representing the results of calculating taxes
pub struct TaxResults {
    /// Amount taken for federal taxes
    pub federal_tax: f32,
    /// Amount taken for state taxes
    pub state_tax: f32,
    /// Amount leftover after taxes + pre-tax removals
    pub net_income: f32,
}

impl TaxResults {
    pub fn new(federal_tax: f32, state_tax: f32, net_income: f32,) -> Self {
        Self {
            federal_tax,
            state_tax,
            net_income
        }
    }
}

impl fmt::Display for TaxResults {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Net Income: {}\nState Taxes: {}\nFederal Taxes: {}",
            self.net_income, self.state_tax, self.federal_tax
        )
    }
}
