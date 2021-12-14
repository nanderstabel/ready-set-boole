use rsb::Parser;

fn print_truth_table(formula: &str) -> String {
    let mut parser = Parser::new();
    if let Ok(table) = parser.truth_table_from(formula) {
        print!("{}", table);
    }
    String::from(formula)
}

#[allow(dead_code)]
fn main() {
    print_truth_table("P>");
}

#[cfg(test)]
mod boolean_evaluation {
    use super::*;

    #[test]
    fn assert_equal() {
        assert_eq!(print_truth_table("A!!"), "A");
        assert_eq!(print_truth_table("AB>"), "A!B|");
        assert_eq!(print_truth_table("AB="), "AB&A!B!&|");

        assert_eq!(print_truth_table("AB&!"), "A!B!|");
        assert_eq!(print_truth_table("AB|!"), "A!B!&");
        assert_eq!(print_truth_table("AB|C&!"), "A!B!&C!|");
    }
}
