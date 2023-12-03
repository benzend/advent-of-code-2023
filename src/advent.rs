use std::fs;

pub fn get_input(day: AdventDay, part: Option<AdventPart>) -> Result<String, std::io::Error> {
    fs::read_to_string(format!("./data/day-{}-part-{}.txt", day, part.unwrap_or_default()))
}


pub struct AdventDay {
    pub value: usize
}

impl AdventDay {
    pub fn new(day: usize) -> Self {
        AdventDay { value: day }
    }
}

impl std::fmt::Display for AdventDay {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.value)
    }
}

pub struct AdventPart {
    pub value: usize
}

impl AdventPart {
    pub fn new(part: usize) -> Self {
        AdventPart { value: part }
    }
}

impl Default for AdventPart {
    fn default() -> Self {
        AdventPart { value: 1 }
    }
}

impl std::fmt::Display for AdventPart {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.value)
    }
}
