use rsb::parser::Parser;

fn eval_set(formula: &str, sets: &[&[i32]]) -> Vec<i32> {
    let mut parser = Parser::new();
    if let Ok(set) = parser.evaluate_set(formula, sets) {
        return set;
    }

    Vec::from([0])
}

#[allow(dead_code)]
fn main() {
    eval_set("AB&", &[&[0, 1, 2], &[0, 3, 4]]);
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
        assert_eq!(
            eval_set(
                "A!B|C&",
                &[&[0, 1, 2, 3], &[2, 3, 4, 5], &[0, 1, 2, 3, 4, 5, 6, 7]]
            ),
            [2, 3, 4, 5, 6, 7]
        );
        assert_eq!(
            eval_set(
                "A!B|C!&",
                &[&[0, 1, 2, 3], &[2, 3, 4, 5, 8], &[0, 1, 2, 3, 4, 5, 6, 7]]
            ),
            [8]
        );
        assert_eq!(
            eval_set(
                "A!B|C!D&&",
                &[
                    &[0, 1, 2, 3],
                    &[2, 3, 4, 5, 8],
                    &[0, 1, 2, 3, 4, 5, 6, 7],
                    &[0, 2, 4, 6, 8, 10]
                ]
            ),
            [8, 10]
        );
        assert_eq!(eval_set("A!A|A!&", &[&[0, 1, 2, 3]]), []);
        assert_eq!(eval_set("A!A|", &[&[0, 1, 2, 3]]), [0, 1, 2, 3]);
    }

    #[test]
    fn intersect() {
        assert_eq!(eval_set("AB&", &[&[0, 1, 2, 3], &[2, 3, 4, 5]]), [2, 3]);
        assert_eq!(eval_set("A!B&", &[&[0, 1, 2, 3], &[2, 3, 4, 5]]), [4, 5]);
        assert_eq!(eval_set("AB!&", &[&[0, 1, 2, 3], &[2, 3, 4, 5]]), [0, 1]);
        assert_eq!(eval_set("A!B!&", &[&[0, 1, 2, 3], &[2, 3, 4, 5]]), []);
    }

    #[test]
    fn union() {
        assert_eq!(
            eval_set("AB|", &[&[0, 1, 2, 3], &[2, 3, 4, 5]]),
            [0, 1, 2, 3, 4, 5]
        );
        assert_eq!(
            eval_set("A!B|", &[&[0, 1, 2, 3], &[2, 3, 4, 5]]),
            [2, 3, 4, 5]
        );
        assert_eq!(
            eval_set("AB!|", &[&[0, 1, 2, 3], &[2, 3, 4, 5]]),
            [0, 1, 2, 3]
        );
        assert_eq!(
            eval_set("A!B!|", &[&[0, 1, 2, 3], &[2, 3, 4, 5]]),
            [0, 1, 4, 5]
        );
    }
}
