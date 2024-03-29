use rsb::parser::Parser;

fn print_truth_table(formula: &str) {
    let mut parser = Parser::new();
    if let Ok(table) = parser.truth_table_from(formula) {
        print!("{}", table);
    }
}

#[allow(dead_code)]
fn main() {
    print_truth_table("AA>");
}

#[cfg(test)]
mod truth_table {
    use super::*;

    #[test]
    fn assert_equal() {
        assert_eq!(
            print_truth_table("CE>11&C&D>&11|C>&11!&F>&C1!|H>&VW^X>&11&YZ&>&CD|XV|>&1F&V!>&11&C=&"),
            ()
        );
    }
}
