fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
    (0..((2i32).pow(set.len() as u32)))
        .map(|c| {
            (0..(set.len()))
                .filter_map(|bit| (c & (1 << bit) != 0).then(|| set[bit]))
                .collect()
        })
        .collect()
}

#[allow(dead_code)]
fn main() {
    println!("{:#?}", powerset(&[0]));
}

#[cfg(test)]
mod powerset {
    use super::*;

    #[test]
    fn assert_equal() {
        assert_eq!(powerset(&[]), [[]]);
        assert_eq!(powerset(&[1]), [[].to_vec(), [1].to_vec()]);
        assert_eq!(
            powerset(&[1, 2]),
            [[].to_vec(), [1].to_vec(), [2].to_vec(), [1, 2].to_vec()]
        );
        assert_eq!(
            powerset(&[1, 2, 3]),
            [
                [].to_vec(),
                [1].to_vec(),
                [2].to_vec(),
                [1, 2].to_vec(),
                [3].to_vec(),
                [1, 3].to_vec(),
                [2, 3].to_vec(),
                [1, 2, 3].to_vec()
            ]
        );
    }

    #[test]
    fn eval_sheet() {
        assert_eq!(powerset(&[]), [[]]);
        assert_eq!(powerset(&[0]), [[].to_vec(), [0].to_vec()]);
        assert_eq!(
            powerset(&[0, 1]),
            [[].to_vec(), [0].to_vec(), [1].to_vec(), [0, 1].to_vec()]
        );
        assert_eq!(
            powerset(&[0, 1, 2]),
            [
                [].to_vec(),
                [0].to_vec(),
                [1].to_vec(),
                [0, 1].to_vec(),
                [2].to_vec(),
                [0, 2].to_vec(),
                [1, 2].to_vec(),
                [0, 1, 2].to_vec()
            ]
        );
    }
}
