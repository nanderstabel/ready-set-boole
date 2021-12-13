use rsb::Parser;
//TODO: https://www.dcode.fr/boolean-truth-table

fn print_truth_table(formula: &str) {
    let mut parser = Parser::new();
    if let Ok(table) = parser.truth_table_from(formula) {
        print!("{}", table);
    }
}

#[allow(dead_code)]
fn main() {
    print_truth_table("AB=");
}

#[cfg(test)]
mod boolean_evaluation {
    use super::*;

    #[test]
    fn assert_equal() {
        assert_eq!(print_truth_table("AB&C|"), ());
    }
}
