use estimate_common::{
    common::{TaxInfo, TaxResults},
    errors::EstimaterResult,
};
use estimate_server::server;

/// Computes taxes given the needed info
pub fn calculate_taxes(info: TaxInfo) -> EstimaterResult<TaxResults> {
    server::calculate_taxes(&info)
}
