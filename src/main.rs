use std::{collections::HashMap, option::Iter};

pub fn gape_shapely(preferences: String) -> String {
    /*
     * Algorithm
     * ---------
     * Proposers go one by one, proposing to their rank list in order
     * and any of the following scenarios could occur:
     *   1. Their preferred partner is unpaired and they pair up
     *   2. Their preferred partner is paired with someone they prefer less, and they become partners
     *   3. Their preferred partner is paired with someone they prefer more, and the proposer move to
     *      their next choice
     */
    let mut input_itr = preferences.lines();
    let participants = input_itr.next().unwrap().split(",");
    let pairings: HashMap<String, Vec<String>> = HashMap::new();

    "".to_string()
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
            B,C"
        .to_string();

        let expected = "B,D\nC,A";

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
            A,D"
        .to_string();

        let expected = "A,B\nD,C";

        let actual = gape_shapely(input);

        assert_eq!(actual, expected);
    }
}
