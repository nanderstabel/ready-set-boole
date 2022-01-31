use rsb::parser::Parser;

fn sat(formula: &str) -> bool {
    let mut parser = Parser::new();
    parser.is_satisfiable(formula)
}

#[allow(dead_code)]
fn main() {
    sat("AA^");
}

#[cfg(test)]
mod sat {
    use super::*;

    #[test]
    fn assert_equal() {
        assert_eq!(sat("AB="), true);
        assert_eq!(sat("AB&"), true);
        assert_eq!(sat("AA!&"), false);
        assert_eq!(sat("AA^"), false);
    }

    #[test]
    fn eval_sheet() {
        assert_eq!(sat("A"), true);
        assert_eq!(sat("A!"), true);
        assert_eq!(sat("AA|"), true);
        assert_eq!(sat("AA&"), true);
        assert_eq!(sat("AA!&"), false);
        assert_eq!(sat("AA^"), false);
        assert_eq!(sat("AB^"), true);
        assert_eq!(sat("AB="), true);
        assert_eq!(sat("AA>"), true);
        assert_eq!(sat("AA!>"), true);

        assert_eq!(sat("ABC||"), true);
        assert_eq!(sat("AB&A!B!&&"), false);
        assert_eq!(sat("ABCDE&&&&"), true);
        assert_eq!(sat("AAA^^"), true);
        assert_eq!(sat("ABCDE^^^^"), true);
    }
}
