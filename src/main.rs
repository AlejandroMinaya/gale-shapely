use std::collections::{HashMap, HashSet};

pub fn gape_shapely(num_participants: usize, participants: Vec<Vec<char>>) -> Vec<Vec<char>> {
    vec![]
}

pub fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairing_up_returns_stable_pairing() {
        /*
         * For the purposes of this test
         * Group #1: A(lina), D(olores)
         * Group #2: B(eto), C(harlie)
         *
         * Rankings:
         * A = ["C", "B"]
         * B = ["A", "D"]
         * D = ["B", "C"]
         * C = ["D", "A"]
         *
         * Proposers: Group #2
         *
         * Expected:
         * (C, A), (B, D)
         *
         */
        let num_participants = 4;
        let participants = vec![
            vec!['A', 'C', 'B'],
            vec!['B', 'A', 'D'],
            vec!['C', 'D', 'A'],
            vec!['D', 'B', 'C'],
        ];

        let expected = vec![vec!['C', 'A'], vec!['B', 'D']];

        let actual = gape_shapely(num_participants, participants);

        assert_eq!(actual, expected);
    }
}
