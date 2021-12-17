use rsb::parser::Parser;

fn sat(formula: &str) -> bool {
    let mut parser = Parser::new();
    parser.is_satisfiable(formula)
}

#[allow(dead_code)]
fn main() {
    sat("AB|C&!");
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
}
