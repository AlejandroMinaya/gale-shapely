use std::collections::{HashMap, HashSet};

pub fn gape_shapely(_preferences: String) -> String {
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
    let mut input_itr = _preferences.lines();
    let all_participants = input_itr.next().unwrap().trim().split(",");

    let mut pairings: HashMap<String, String> = HashMap::new();
    let mut preferences: HashMap<String, Vec<String>> = HashMap::new();

    for participant in all_participants.clone() {
        preferences.insert(
            participant.to_string(),
            input_itr
                .next()
                .unwrap()
                .trim()
                .split(",")
                .map(|a| a.to_string())
                .collect(),
        );
        pairings.insert(participant.to_string(), "".to_string());
    }
    println!("Preferences {preferences:?}");
    println!("Pairings {pairings:?}");

    while pairings.values().any(|partner| partner.is_empty()) {
        for suitor in all_participants.clone() {
            let choices = &preferences[suitor];
            println!("{suitor} Choices {choices:?}");

            let boo = choices.first().unwrap();
            println!("OPP - Pairings {pairings:?} {boo}");
            let opp = pairings[boo].clone();

            // 1. Chosen partner is available
            if opp.is_empty() {
                pairings.insert(boo.to_string(), suitor.to_string());
                pairings.insert(suitor.to_string(), boo.to_string());
                println!("{suitor} just hooked up with {boo}!");
                continue;
            }

            let suitor_pos = preferences[boo].iter().position(|peep| *peep == *suitor);
            let opp_pos = preferences[boo].iter().position(|peep| *peep == *opp);

            if suitor_pos < opp_pos {
                pairings.insert(boo.to_string(), suitor.to_string());
                pairings.insert(suitor.to_string(), boo.to_string());
                pairings.insert(opp.to_string(), "".to_string());
                println!("{suitor} just stole {boo} from {opp}!");
            }
        }
    }
    let mut result = String::new();
    let mut reported_participants: HashSet<String> = HashSet::new();
    for _participant in all_participants.clone() {
        let participant = _participant.to_string();
        let partner = pairings[&participant].clone();
        if reported_participants.contains(&participant) || reported_participants.contains(&partner)
        {
            continue;
        }
        result = format!("{result}{participant},{partner}\n");
        reported_participants.insert(participant);
        reported_participants.insert(partner);
    }

    result.trim().to_string()
}

pub fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_two_gale_shapely_returns_stable_pairing() {
        /*
         * For the purposes of this test
         * Group #1: A(nna), D(olores)
         * Group #2: B(eto), C(harlie)
         *
         * Rankings:
         * A = ["C", "B"]
         * B = ["A", "D"]
         * D = ["B", "C"]
         * C = ["A", "D"]
         *
         * Proposers: Group #2
         *
         * Expected:
         * (B, D), (C, A)
         *
         */
        let preferences = "B,C,A,D\nA,D\nA,D\nC,B\nB,C".to_string();

        let expected = "B,D\nC,A";

        let actual = gape_shapely(preferences);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_group_one_gale_shapely_returns_stable_pairing() {
        /*
         * For the purposes of this test
         * Group #1: A(nna), D(olores)
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
        let input = "A,D,C,B\nC,B\nC,B\nD,A\nA,D".to_string();

        let expected = "A,B\nD,C";

        let actual = gape_shapely(input);

        assert_eq!(actual, expected);
    }
}
