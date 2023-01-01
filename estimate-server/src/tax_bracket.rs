/// Implements the concept of tax brackets. Usable for both state and federal
/// income taxes.
use std::cmp::Ordering;
use estimate_common::{
    errors::EstimaterResult,
};


/// Struct representing all tax brackets that exist.
struct TaxBrackets {
    brackets: Vec<BracketInfo>,
}

/// Struct representing an individual tax bracket
#[derive(Debug)]
struct BracketInfo {
    /// The lower limit (inclusive) that this tax bracket is a part pf
    bracket_min: i64,
    /// The lower limit (non-envlusive) that this tax bracket is part of
    bracket_max: i64,
    /// The percentage tax rate that is applied to the amount within this tax
    /// bracket. i.e. this rate gets applied to `value` in `lower_limit` <= `value` < `upper_limit`
    percentage_rate: f32,

    /// The overall taxes paid through all the previous tax brackets
    cumulative_previous_tax: f32,
}

impl TaxBrackets {
    pub fn new() -> Self {
        Self {
            brackets: Vec::new(),
        }
    }

    /// Adds a bracket to the list of brackets. Maintaining the sortedness of the brackets.
    /// # Return
    /// * Ok if the bracket is added successfully
    pub fn add_bracket(&mut self, new_bracket: BracketInfo) -> EstimaterResult<()> {
        self.brackets.push(new_bracket);
        self.brackets.sort();
        Ok(())
    }
}

impl BracketInfo {
    pub fn new(
        bracket_min: i64,
        bracket_max: i64,
        percent_rate: f32,
        cumulative_previous_tax: f32,
    ) -> Self {
        Self {
            bracket_min,
            bracket_max,
            percentage_rate: percent_rate,
            cumulative_previous_tax: cumulative_previous_tax,
        }
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
            && self.percentage_rate == other.percentage_rate
            && self.cumulative_previous_tax == other.cumulative_previous_tax
    }
}

impl Eq for BracketInfo {}
