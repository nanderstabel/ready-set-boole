use itertools::Itertools;

fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
    let mut powerset: Vec<Vec<i32>> = (0..=set.len())
        .tuple_combinations::<(_, _)>()
        .map(|(start, end)| set[start..end].to_vec())
        .collect();
    powerset.push(Vec::from([]));
    powerset
}

#[allow(dead_code)]
fn main() {
    println!("{:#?}", powerset(&[1, 2, 3]));
}

#[cfg(test)]
mod powerset {
    use super::*;

    #[test]
    fn assert_equal() {
        assert_eq!(powerset(&[]), [[]]);
        assert_eq!(powerset(&[1]), [[1].to_vec(), [].to_vec()]);
        assert_eq!(
            powerset(&[1, 2]),
            [[1].to_vec(), [1, 2].to_vec(), [2].to_vec(), [].to_vec()]
        );
        assert_eq!(
            powerset(&[1, 2, 3]),
            [
                [1].to_vec(),
                [1, 2].to_vec(),
                [1, 2, 3].to_vec(),
                [2].to_vec(),
                [2, 3].to_vec(),
                [3].to_vec(),
                [].to_vec()
            ]
        );
    }
}
