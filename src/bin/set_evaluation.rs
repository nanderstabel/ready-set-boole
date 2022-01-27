use rsb::parser::Parser;

fn eval_set(formula: &str, sets: &[&[i32]]) -> Vec<i32> {
    let mut parser = Parser::new();
    if let Ok(set) = parser.evaluate_set(formula, sets) {
        println!("{:?}", set);
        return set;
    }

    Vec::from([0])
}

#[allow(dead_code)]
fn main() {
    println!("{:#?}", eval_set("AB&", &[&[0, 1, 2], &[0, 3, 4]]));
}

#[cfg(test)]
mod set_evaluation {
    use super::*;

    #[test]
    fn assert_equal() {
        assert_eq!(eval_set("AB&", &[&[0, 1, 2], &[0, 3, 4]]), [0]);
        assert_eq!(
            eval_set("AB|", &[&[0, 1, 2], &[3, 4, 5]]),
            [0, 1, 2, 3, 4, 5]
        );
        assert_eq!(eval_set("A!", &[&[0, 1, 2]]), []);
    }
}
