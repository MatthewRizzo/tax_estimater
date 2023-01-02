use serde::Deserialize;
use serde_valid::Validate;
use std::cmp::Ordering;
/// Implements the concept of tax brackets. Usable for both state and federal
/// income taxes.
use std::fmt;

use estimate_common::errors::{BracketErrors, EstimaterErrors, EstimaterResult};
/// Struct representing all tax brackets that exist.
#[derive(Debug, Deserialize)]
pub(crate) struct TaxBrackets {
    brackets: Vec<BracketInfo>,
}

/// Struct representing an individual tax bracket
#[derive(Debug, Deserialize, Validate)]
pub(crate) struct BracketInfo {
    /// The lower limit (inclusive) that this tax bracket is a part pf
    pub bracket_min: u64,
    /// The lower limit (non-envlusive) that this tax bracket is part of
    pub bracket_max: u64,
    /// The percentage tax rate that is applied to the amount within this tax
    /// bracket. i.e. this rate gets applied to `value` in `lower_limit` <= `value` < `upper_limit`.
    /// Note: ranges 0 <= `tax_rate` <= 1
    #[validate(minimum = 0.0)]
    #[validate(maximum = 1.0)]
    pub tax_rate: f64,

    /// The overall taxes paid through all the previous tax brackets (excluding this one).
    /// This is the total amount of taxes that are required by all brackets BEFORE
    /// this one.
    cumulative_previous_tax: Option<f64>,
}

impl TaxBrackets {
    /// Resorts all brackets to be in the correct order
    pub fn sort_brackets(&mut self) {
        self.brackets.sort();
    }

    /// For all brackets, tabulates the cumulative amount of taxes across all
    /// previous brackets UP TO (but not including) this one.
    ///
    /// # Precondition
    /// The brackets are ordered by their bounds
    ///
    pub fn tabulate_cumulative_taxes(&mut self) {
        let mut total_cumalitive: f64 = 0.0;

        for bracket in self.brackets.iter_mut() {
            let cur_max_tax = bracket.calculate_bracket_tabulated_maximum();
            if cur_max_tax.is_err() {
                panic!(
                    "Calulating the tabulated amount for the bracket failed.\n{:?}",
                    cur_max_tax.err()
                );
            };

            let opt_existing_cumalitive = bracket.cumulative_previous_tax;
            if let Some(existing_cumalitive) = opt_existing_cumalitive {
                if existing_cumalitive != total_cumalitive {
                    panic!("Tabulating bracket costs failed. Cumalitive given: {}, calculated = {}. For bracket {}",
                    existing_cumalitive, total_cumalitive, bracket)
                }
            }
            println!("previous: {}", total_cumalitive);
            total_cumalitive =
                BracketInfo::round_to_hundredths(total_cumalitive + cur_max_tax.unwrap());

            bracket.cumulative_previous_tax = Some(total_cumalitive);
        }
    }

    pub(crate) fn validate_all_brackets(&self) -> EstimaterResult<()> {
        for bracket in self.brackets.iter() {
            // BracketInfo::validate_new_bracket(bracket.bracket_min, bracket.bracket_max, bracket.tax_rate)
            let val_res = BracketInfo::validate_new_bracket(
                bracket.bracket_min,
                bracket.bracket_max,
                bracket.tax_rate,
            );

            if let Err(err) = val_res {
                Err(EstimaterErrors::BracketError(err))?
            }
        }

        Ok(())
    }
}

impl fmt::Display for TaxBrackets {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for bracket in self.brackets.iter() {
            write!(f, "{}", bracket)?;
        }
        write!(f, "")
    }
}

impl fmt::Display for BracketInfo {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bracket_min = {}. ", self.bracket_min)?;
        write!(f, "bracket_max = {}. ", self.bracket_max)?;
        write!(f, "tax_rate = {}. ", self.tax_rate)?;
        if self.cumulative_previous_tax.is_some() {
            write!(
                f,
                "cumulative_previous_tax = {}. ",
                self.cumulative_previous_tax.unwrap()
            )?;
        }
        writeln!(f)
    }
}

impl BracketInfo {
    #[allow(dead_code)]
    pub fn new(
        bracket_min: u64,
        bracket_max: u64,
        tax_rate: f64,
        cumulative_previous_tax: Option<f64>,
    ) -> EstimaterResult<Self> {
        let validation_res = Self::validate_new_bracket(bracket_min, bracket_max, tax_rate);
        match validation_res {
            Err(err) => Err(EstimaterErrors::BracketError(err)),
            Ok(_) => Ok(Self {
                bracket_min,
                bracket_max,
                tax_rate,
                cumulative_previous_tax,
            }),
        }
    }

    pub(crate) fn validate_new_bracket(
        bracket_min: u64,
        bracket_max: u64,
        tax_rate: f64,
    ) -> std::result::Result<(), BracketErrors> {
        if !(0.0..=1.0).contains(&tax_rate) {
            Err(BracketErrors::TaxRateError(
                "Tax rate not within [0, 1]".to_string(),
            ))
        } else if bracket_min >= bracket_max {
            let err_msg = format!(
                "Bracket minimimum {} is >= bracket maximum ({})",
                bracket_min, bracket_max
            );
            Err(BracketErrors::RangeError(err_msg))
        } else {
            Ok(())
        }
    }

    /// Calculates the taxes for the current bracket.
    ///
    /// # Return
    ///
    /// * The tax amount if successful
    /// * `EstimaterErrors::BracketError` when the income is outside the bounds of taxable range
    pub fn calculate_bracket_taxes(&self, taxable_income: u64) -> EstimaterResult<f64> {
        let tax = self.tax_rate * taxable_income as f64;
        Ok(Self::round_to_hundredths(tax))
    }

    /// Calculates the (tabulated) maximum tax resulting from this tax bracket. i.e. the graduated
    /// taxes from this bracket if it is exceeded.
    pub(self) fn calculate_bracket_tabulated_maximum(&self) -> EstimaterResult<f64> {
        let taxable_amount = self.bracket_max - self.bracket_min;
        self.calculate_bracket_taxes(taxable_amount)
    }

    /// A lot of tax documents only use 2 decimal sig-figs. To align our
    /// calculations, the same is being repeated here.
    pub(self) fn round_to_hundredths(value: f64) -> f64 {
        (value * 100.0).round() / 100.0
    }
}

impl Ord for BracketInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.bracket_min.cmp(&other.bracket_max)
    }
}

impl PartialOrd for BracketInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BracketInfo {
    fn eq(&self, other: &Self) -> bool {
        self.bracket_min == other.bracket_min
            && self.bracket_max == other.bracket_max
            && self.tax_rate == other.tax_rate
            && self.cumulative_previous_tax == other.cumulative_previous_tax
    }
}

impl Eq for BracketInfo {}

#[cfg(test)]
mod tests {
    // TODO: write tests here
    // #[test]
}
