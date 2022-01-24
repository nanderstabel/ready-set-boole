use std::fmt;

#[derive(PartialEq)]
pub struct TruthTable {
    pub variables: Vec<char>,
    pub results: Vec<bool>,
}

impl TruthTable {
    pub fn new() -> Self {
        TruthTable {
            variables: Vec::new(),
            results: Vec::new(),
        }
    }
}

impl fmt::Display for TruthTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = self.variables.len();
        for v in &self.variables {
            write!(f, "| {} ", v)?;
        }
        write!(f, "| = |\n")?;
        write!(f, "{}|\n", "|---".repeat(len + 1))?;
        for (i, result) in self.results.iter().enumerate() {
            for b in 0..len {
                write!(
                    f,
                    "| {} ",
                    if i & (1 << (len - 1 - b)) == 0 { 0 } else { 1 }
                )?
            }
            write!(f, "| {} |\n", if *result { 1 } else { 0 })?;
        }
        write!(f, "")
    }
}

impl fmt::Debug for TruthTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
