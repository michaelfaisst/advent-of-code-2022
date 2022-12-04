use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub struct Assignment {
    sector_start: usize,
    sector_end: usize,
}

impl FromStr for Assignment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once("-").unwrap();

        Ok(Assignment {
            sector_start: start.parse().unwrap(),
            sector_end: end.parse().unwrap(),
        })
    }
}

impl Assignment {
    pub fn contains_other_assignment(&self, assignment: &Assignment) -> bool {
        return self.sector_start <= assignment.sector_start
            && self.sector_end >= assignment.sector_end;
    }

    pub fn overlaps(&self, assignment: &Assignment) -> bool {
        return (self.sector_start >= assignment.sector_start
            && self.sector_start <= assignment.sector_end)
            || (self.sector_end >= assignment.sector_start
                && self.sector_end <= assignment.sector_end);
    }
}
