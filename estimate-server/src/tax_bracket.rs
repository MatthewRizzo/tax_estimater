/// Implements the concept of tax brackets. Usable for both state and federal
/// income taxes.
use serde::Deserialize;
use serde_valid::Validate;
use std::{cmp::Ordering, fmt, fs::File, io::BufReader, path::PathBuf};

use estimate_common::errors::{BracketErrors, EstimaterErrors, EstimaterResult};

type BracketResult<T> = std::result::Result<T, BracketErrors>;

/// Struct representing all tax brackets that exist.
#[derive(Debug, Deserialize)]
pub(crate) struct TaxBrackets {
    brackets: Vec<BracketInfo>,
}

/// Struct representing an individual tax bracket
#[derive(Debug, Deserialize, Validate)]
pub(crate) struct BracketInfo {
    /// The lower limit (inclusive) that this tax bracket is a part of.
    /// CANNOT overlap with max of previous!
    pub bracket_min: u64,
    /// The lower limit (inclusive) that this tax bracket is part of
    /// CANNOT overlap with min of next bracket!
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
    /// Attempts to read from the json containing tax bracket info.
    ///
    /// # Return
    ///
    /// * Error if file doesn't exist (or something else)
    /// * Success: TaxBracket instance with sorted tax brackets.
    pub(crate) fn from_bracket_json(path: PathBuf) -> EstimaterResult<Self> {
        let file = File::open(&path);
        if let Ok(opened_file) = file {
            let read_buffer = BufReader::new(opened_file);
            let mut brackets: TaxBrackets = serde_json::from_reader(read_buffer)
                .map_err(EstimaterErrors::SerdeDeserializeError)?;
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
            total_cumalitive =
                BracketInfo::round_to_hundredths(total_cumalitive + cur_max_tax.unwrap());

            bracket.cumulative_previous_tax = Some(total_cumalitive);
        }
    }

    /// # Pre-condition
    /// The brackets are sorted!
    pub(crate) fn validate_all_brackets(&self) -> EstimaterResult<()> {
        for (bracket_idx, bracket) in self.brackets.iter().enumerate() {
            // BracketInfo::validate_new_bracket(bracket.bracket_min, bracket.bracket_max, bracket.tax_rate)
            let val_res = BracketInfo::validate_new_bracket(
                bracket.bracket_min,
                bracket.bracket_max,
                bracket.tax_rate,
            );

            if let Err(err) = val_res {
                Err(EstimaterErrors::BracketError(err))?
            }

            if bracket_idx > 0 && bracket.check_for_bracket_overlap(&self.brackets[bracket_idx - 1])
            {
                let err_msg = format!(
                    "Overlap of bracket {bracket} and {}",
                    self.brackets[bracket_idx - 1]
                );
                Err(EstimaterErrors::BracketError(BracketErrors::OverlapError(
                    err_msg,
                )))?
            }
        }

        Ok(())
    }

    /// Calculate the total amount of taxes that need to be
    /// paid on a given gross income.
    ///
    /// # Params
    /// * `self` - The tax bracket info needed.
    /// * `taxable_income` - The taxable income to apply the bracket too
    ///
    /// # Return
    /// The amount to pay in taxes
    pub(crate) fn calculate_tax_amount(&self, taxable_income: f64) -> EstimaterResult<f64> {
        let tax_bracket_index = self.determine_correct_bracket(&taxable_income)?;
        let bracket_info = &self.brackets[tax_bracket_index];
        bracket_info.calculate_bracket_taxes(taxable_income)
    }

    /// Given a taxable income. Determines the correct top bracket to put it in.
    ///
    /// # Result
    /// * The bracket index if it exists
    /// * `Err` - If the income does not have a valid bracket
    fn determine_correct_bracket(&self, taxable_income: &f64) -> BracketResult<usize> {
        self.brackets
            .iter()
            .position(|cur_bracket| {
                taxable_income >= &(cur_bracket.bracket_min as f64)
                    && taxable_income <= &(cur_bracket.bracket_max as f64)
            })
            .ok_or_else(|| {
                BracketErrors::LargeIncomeError(format!(
                    "The income {taxable_income} does not fit in ANY tax bracket"
                ))
            })
    }
}

impl fmt::Display for TaxBrackets {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for bracket in self.brackets.iter() {
            write!(f, "{}", bracket)?;
        }
        writeln!(f)
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

    /// Checks if there is overlap between 2 brackets from the perspective of self.
    /// Overlap is found when:
    /// The min of 1 bracket is between the min and max of another
    /// OR
    /// The max of 1 bracket is between the min and max of another
    pub(crate) fn check_for_bracket_overlap(&self, other: &Self) -> bool {
        let is_min_within_other: bool =
            self.bracket_min >= other.bracket_min && self.bracket_min <= other.bracket_max;
        let is_max_within_other: bool =
            self.bracket_max >= other.bracket_min && self.bracket_max <= other.bracket_max;
        is_min_within_other || is_max_within_other
    }

    /// Calculates the taxes for the current bracket.
    ///
    /// # Return
    ///
    /// * The tax amount if successful
    /// * `EstimaterErrors::BracketError` when the income is outside the bounds of taxable range
    pub fn calculate_bracket_taxes(&self, taxable_income: f64) -> EstimaterResult<f64> {
        let tax = self.tax_rate * taxable_income;
        Ok(Self::round_to_hundredths(tax))
    }

    /// Calculates the (tabulated) maximum tax resulting from this tax bracket. i.e. the graduated
    /// taxes from this bracket if it is exceeded.
    fn calculate_bracket_tabulated_maximum(&self) -> EstimaterResult<f64> {
        let taxable_amount = self.bracket_max - self.bracket_min;
        self.calculate_bracket_taxes(taxable_amount as f64)
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
    use super::*;

    /// Helper function to assert the result is Ok() and matches the given param
    fn help_assert_result<T, ErrorType>(
        result_to_check: std::result::Result<T, ErrorType>,
        expected_res: T,
        additional_fail_msg: &str,
    ) where
        T: PartialEq<T>,
        T: std::fmt::Display,
        ErrorType: std::fmt::Display,
        ErrorType: std::fmt::Debug,
    {
        match result_to_check {
            Err(err) => assert!(
                false,
                "{}",
                format!(
                    "Expected {expected_res}, got err: {:?} for {additional_fail_msg}",
                    err
                )
            ),
            Ok(res) => {
                if res == expected_res {
                    assert!(true)
                } else {
                    let err_msg =
                        format!("Expected {expected_res}, got {res} for {additional_fail_msg}");
                    assert!(false, "{err_msg}")
                }
            }
        }
    }

    // helper to generate some brackets from a json string
    fn help_make_test_brackets() -> TaxBrackets {
        let bracket_json_str = r#"{
            "brackets": [
                {
                    "bracket_max": 10275,
                    "bracket_min": 0,
                    "cumulative_previous_tax": 0.0,
                    "tax_rate": 0.1
                  },
                  {
                    "bracket_max": 41776,
                    "bracket_min": 10276,
                    "cumulative_previous_tax": 1027.5,
                    "tax_rate": 0.12
                  }
            ]
        }"#;
        return serde_json::from_str(bracket_json_str).unwrap();
    }

    #[test]
    fn test_check_for_bracket_overlap() {
        let non_overlap_bracket = help_make_test_brackets();
        let overlap_bracket_json_str = r#"{
            "brackets": [
                {
                    "bracket_max": 10275,
                    "bracket_min": 0,
                    "cumulative_previous_tax": 0.0,
                    "tax_rate": 0.1
                  },
                  {
                    "bracket_max": 30000,
                    "bracket_min": 500,
                    "cumulative_previous_tax": 1027.5,
                    "tax_rate": 0.12
                  }
            ]
        }"#;
        let overlap_bracket: TaxBrackets = serde_json::from_str(overlap_bracket_json_str).unwrap();

        assert!(
            non_overlap_bracket.validate_all_brackets().is_ok(),
            "Validating all brackets (without overlap), failed"
        );
        assert!(
            overlap_bracket.validate_all_brackets().is_err(),
            "Valdiating brackets with overlap did not error as expected"
        );
    }

    #[test]
    fn test_determine_correct_bracket() {
        let brackets = help_make_test_brackets();
        help_assert_result(brackets.determine_correct_bracket(&0.0), 0, "input of 0.0");
        help_assert_result(
            brackets.determine_correct_bracket(&1000.0),
            0,
            "input of 1000.0",
        );
        help_assert_result(
            brackets.determine_correct_bracket(&10000.0),
            0,
            "input of 10000.0",
        );
        help_assert_result(
            brackets.determine_correct_bracket(&10275.0),
            0,
            "input of 10275.0",
        );
        help_assert_result(
            brackets.determine_correct_bracket(&10276.0),
            1,
            "input of 10276.0",
        );
        help_assert_result(
            brackets.determine_correct_bracket(&15000.0),
            1,
            "input of 15000.0",
        );
    }

    #[test]
    fn test_calculate_taxes() {
        let brackets = help_make_test_brackets();
        help_assert_result(brackets.calculate_tax_amount(0.0), 0.0, "input of 0.0");
        help_assert_result(
            brackets.calculate_tax_amount(10275.0),
            1027.5,
            "input of 10275.0",
        );
    }
}
