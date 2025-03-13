use std::collections::{HashMap, HashSet};

pub fn main() {}

pub fn pair_up(participants: HashMap<char, HashSet<char>>) -> HashSet<(char, char)> {
    HashSet::new()
}

struct Party<'a> {
    preferences: &'a str,
}

impl Party<'_> {
    pub fn how_interested_in(&self, other: char) -> usize {
        for (i, c) in self.preferences.chars().enumerate() {
            if c == other {
                return i + 1;
            }
        }
        usize::MAX
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::*;
    const ALINA: Party = Party { preferences: "BC" };

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
        let participants: HashMap<char, HashSet<char>> = HashMap::from([
            ('A', HashSet::from(['C', 'B'])),
            ('B', HashSet::from(['A', 'D'])),
            ('C', HashSet::from(['D', 'A'])),
            ('D', HashSet::from(['B', 'C'])),
        ]);

        let expected = HashSet::from([('C', 'A'), ('B', 'D')]);

        let actual = pair_up(participants);

        assert_eq!(actual, expected);
    }
}
