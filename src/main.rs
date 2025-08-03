use std::{collections::HashMap, option::Iter};

pub fn gape_shapely(preferences: &str) -> Vec<Vec<&str>> {
    /*
     * Algorithm
     * ---------
     * Proposers go one by one, proposing to their rank list in order
     * and any of the following scenarios could occur:
     *   1. Their chosen partner is unpaired and they become partners
     *   2. Their chosen partner is paired with someone they prefer less, and they become partners
     *   3. Their chosen partner is paired with someone they prefer more, and the proposer move to
     *      their next choice
     */
    let preferences = _read_preferences(preferences);

    vec![vec![], vec![]]
}

fn _read_preferences(preferences: &str) -> HashMap<&str, Vec<&str>> {
    let mut preferences_lines = preferences.trim().lines();
    let participants = preferences_lines.next().unwrap().trim().split(",");

    let mut ranking = HashMap::new();
    for participant in participants {
        let participant_ranking = preferences_lines.next().unwrap().trim().split(",");
        ranking.insert(participant, participant_ranking.collect());
    }
    ranking
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
         * (B, D), (C, A)
         *
         */
        let preferences = "B,C,A,D
            A,D
            D,A
            C,B
            B,C";

        let expected = vec![vec!["B", "D"], vec!["C", "A"]];

        let actual = gape_shapely(preferences);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_pairing_up_returns_stable_pairing_for_different_preferences() {
        /*
         * For the purposes of this test
         * Group #1: A(lina), D(olores)
         * Group #2: B(eto), C(harlie)
         *
         * Rankings:
         * A = ["C", "B"]
         * B = ["A", "D"]
         * D = ["C", "B"]
         * C = ["D", "A"]
         *
         * Proposers: Group #1
         *
         * Expected:
         * (A, B), (D, C)
         *
         */
        let input = "A,D,C,B
            C,B
            C,B
            D,A
            A,D";

        let expected = vec![vec!["A", "B"], vec!["D", "C"]];

        let actual = gape_shapely(input);

        assert_eq!(actual, expected);
    }
}
