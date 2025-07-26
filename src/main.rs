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
    let preferences = _get_preferences_by_participant(preferences);
    let participants = preferences.keys();
    let pairings = _setup_pairings_map(participants);

    let proposer_count = 0;

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

fn _setup_pairings_map<'a>(participants: impl Iterator<Item = &'a str>) -> HashMap<&str, &str> {
    let mut initial_pairings: HashMap<&str, &str> = HashMap::new();

    for participant in participants {
        initial_pairings.insert(participant.clone(), "");
    }

    initial_pairings
}

fn _get_next_up_for<'a>(
    pairings: &HashMap<&str, &str>,
    preferences: &'a mut HashMap<&'a str, Vec<&str>>,
    participant: &'a str,
) -> &'a str {
    match preferences.get(participant).unwrap().first() {
        Some(next_up) => {
            if _is_single(pairings, next_up) {
                return next_up;
            }
            let new_preferences = preferences.get_mut(participant).unwrap().split_off(1);
            preferences.insert(participant, new_preferences);

            _get_next_up_for(pairings, preferences, participant)
        }
        _ => "",
    }
}

fn _match<'a>(
    pairings: &'a mut HashMap<&'a str, &'a str>,
    preferences: &'a mut HashMap<&'a str, Vec<&str>>,
    proposing: &'a str,
    receiving: &'a str,
) -> bool {
    if _is_single(pairings, receiving)
        || _get_next_up_for(pairings, preferences, receiving) == proposing
    {
        let prev_proposing_partner = pairings.get(proposing).unwrap();
        if prev_proposing_partner.is_empty() {
            pairings.insert(prev_proposing_partner, "");
        }
        let prev_receiving_partner = pairings.get(receiving).unwrap();
        if prev_receiving_partner.is_empty() {
            pairings.insert(prev_receiving_partner, "");
        }

        pairings.insert(proposing, receiving);
        pairings.insert(receiving, proposing);

        return true;
    }
    false
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
        let mut pairings = HashMap::from([("A", ""), ("B", ""), ("C", ""), ("D", "")]);
        let mut preferences = HashMap::from([
            ("B", vec!["A", "D"]),
            ("C", vec!["D", "A"]),
            ("A", vec!["C", "B"]),
            ("D", vec!["B", "C"]),
        ]);
        assert!(_match(&mut pairings, &mut preferences, "B", "A"));
    }

    #[test]
    fn test_is_a_match_when_receiving_end_also_prefers_them() {
        let mut pairings = HashMap::from([("A", "B"), ("B", "A"), ("C", ""), ("D", "")]);
        let mut preferences = HashMap::from([
            ("B", vec!["A", "D"]),
            ("C", vec!["A", "D"]),
            ("A", vec!["C", "B"]),
            ("D", vec!["B", "C"]),
        ]);
        assert!(_match(&mut pairings, &mut preferences, "C", "A"));
    }
    #[test]
    fn test_is_no_match_when_receiving_end_is_with_preferred_party() {
        let mut pairings = HashMap::from([("A", "C"), ("B", "D"), ("C", "A"), ("D", "B")]);
        let mut preferences = HashMap::from([
            ("B", vec!["A", "D"]),
            ("C", vec!["D", "A"]),
            ("A", vec!["C", "B"]),
            ("D", vec!["B", "C"]),
        ]);
        assert!(!_match(&mut pairings, &mut preferences, "C", "D"));
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
    fn test_get_initial_pairings() {
        let preferences = HashMap::from([
            ("B", vec!["A", "D"]),
            ("C", vec!["D", "A"]),
            ("A", vec!["C", "B"]),
            ("D", vec!["B", "C"]),
        ]);
        assert_eq!(
            _setup_pairings_map(&preferences),
            HashMap::from([("B", ""), ("C", ""), ("A", ""), ("D", "")])
        )
    }

    #[test]
    fn test_get_next_up_preference_for_given_member() {
        let pairings = HashMap::from([("A", ""), ("B", ""), ("C", ""), ("D", "")]);
        let mut preferences = HashMap::from([
            ("B", vec!["A", "D"]),
            ("C", vec!["D", "A"]),
            ("A", vec!["C", "B"]),
            ("D", vec!["B", "C"]),
        ]);
        assert_eq!(_get_next_up_for(&pairings, &mut preferences, "B"), "A");
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
        assert_eq!(_get_next_up_for(&pairings, &mut preferences, "C"), "D");
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
