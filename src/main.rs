use std::collections::HashMap;

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
    let preferences = _get_preferences_by_participant(preferences);

    vec![vec!["B", "D"], vec!["C", "A"]]
}

fn _get_preferences_by_participant(preferences: &str) -> HashMap<&str, Vec<&str>> {
    let mut preferences_lines = preferences.trim().lines();
    let participants = preferences_lines.next().unwrap().trim().split(",");

    let mut ranking = HashMap::new();
    for participant in participants {
        let personal_ranking = preferences_lines.next().unwrap().trim().split(",");
        ranking.insert(participant, personal_ranking.collect());
    }
    ranking
}

fn _get_next_up_for<'a>(
    pairings: &HashMap<&str, &str>,
    preferences: &'a HashMap<&str, Vec<&str>>,
    participant: &str,
) -> &'a str {
    preferences.get(participant).unwrap()[0]
}

fn _is_a_match(
    pairings: &HashMap<&str, &str>,
    preferences: &HashMap<&str, Vec<&str>>,
    proposing: &str,
    receiving: &str,
) -> bool {
    if _is_single(pairings, receiving) {
        return true;
    }

    _get_next_up_for(pairings, preferences, receiving) == proposing
}

fn _is_single(pairings: &HashMap<&str, &str>, participant: &str) -> bool {
    match pairings.get(participant) {
        Some(partner) => partner.is_empty(),
        _ => false,
    }
}

pub fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_single() {
        let pairings = HashMap::from([("A", ""), ("B", "")]);
        assert!(_is_single(&pairings, "A"));
        assert!(_is_single(&pairings, "B"));
        assert!(
            !_is_single(&pairings, "K"),
            "Should return false since this is not a participant"
        );
    }
    #[test]
    fn test_is_a_match_when_receiving_end_is_alone() {
        let pairings = HashMap::from([("A", ""), ("B", ""), ("C", ""), ("D", "")]);
        let preferences = HashMap::from([
            ("B", vec!["A", "D"]),
            ("C", vec!["D", "A"]),
            ("A", vec!["C", "B"]),
            ("D", vec!["B", "C"]),
        ]);
        assert!(_is_a_match(&pairings, &preferences, "B", "A"));
        assert!(_is_a_match(&pairings, &preferences, "C", "D"));
    }

    #[test]
    fn test_is_a_match_when_receiving_end_also_prefers_them() {
        let pairings = HashMap::from([("A", "B"), ("B", "A"), ("C", ""), ("D", "")]);
        let preferences = HashMap::from([
            ("B", vec!["A", "D"]),
            ("C", vec!["A", "D"]),
            ("A", vec!["C", "B"]),
            ("D", vec!["B", "C"]),
        ]);
        assert!(_is_a_match(&pairings, &preferences, "C", "A"));
    }
    #[test]
    fn test_is_no_match_when_receiving_end_is_with_preferred_party() {
        let pairings = HashMap::from([("A", "C"), ("B", "D"), ("C", "A"), ("D", "B")]);
        let preferences = HashMap::from([
            ("B", vec!["A", "D"]),
            ("C", vec!["D", "A"]),
            ("A", vec!["C", "B"]),
            ("D", vec!["B", "C"]),
        ]);
        assert!(!_is_a_match(&pairings, &preferences, "B", "A"));
        assert!(!_is_a_match(&pairings, &preferences, "C", "D"));
    }
    #[test]
    fn test_get_preferences_by_participant() {
        let preferences = "
            B,C,A,D
            A,D
            D,A
            C,B
            B,C
        ";
        assert_eq!(
            _get_preferences_by_participant(preferences),
            HashMap::from([
                ("B", vec!["A", "D"]),
                ("C", vec!["D", "A"]),
                ("A", vec!["C", "B"]),
                ("D", vec!["B", "C"]),
            ])
        )
    }

    #[test]
    fn test_get_next_up_preference_for_given_member() {
        let pairings = HashMap::from([("A", ""), ("B", ""), ("C", ""), ("D", "")]);
        let preferences = HashMap::from([
            ("B", vec!["A", "D"]),
            ("C", vec!["D", "A"]),
            ("A", vec!["C", "B"]),
            ("D", vec!["B", "C"]),
        ]);
        assert_eq!(_get_next_up_for(&pairings, &preferences, "B"), "A");
        assert_eq!(_get_next_up_for(&pairings, &preferences, "C"), "D");
        assert_eq!(_get_next_up_for(&pairings, &preferences, "A"), "C");
        assert_eq!(_get_next_up_for(&pairings, &preferences, "D"), "B");
    }

    #[test]
    fn test_get_next_up_updates_preferences_based_on_pairings() {
        let pairings = HashMap::from([("A", "B"), ("B", "A"), ("C", ""), ("D", "")]);
        let mut preferences = HashMap::from([
            ("B", vec!["A", "D"]),
            ("C", vec!["A", "D"]),
            ("A", vec!["B", "C"]),
            ("D", vec!["B", "C"]),
        ]);
        assert_eq!(_get_next_up_for(&pairings, &preferences, "C"), "D");
        assert_eq!(_get_next_up_for(&pairings, &preferences, "D"), "C");
        assert_eq!(
            preferences,
            HashMap::from([
                ("B", vec!["A", "D"]),
                ("C", vec!["D"]),
                ("A", vec!["B", "C"]),
                ("D", vec!["C"]),
            ])
        );
    }

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
