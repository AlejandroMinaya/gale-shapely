pub fn main() {}

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
    use super::*;
    const ALINA: Party = Party { preferences: "BC" };

    #[test]
    fn test_can_return_other_parties_interested() {
        /* For the purposes of this test
         * Group #1: A(lina), D(olores)
         * Group #2: B(eto), C(harlie)
         */
        assert_eq!(ALINA.how_interested_in('B'), 1);
        assert_eq!(ALINA.how_interested_in('C'), 2);
    }

    #[test]
    fn test_returns_the_maximum_for_unknown_parties() {
        assert_eq!(ALINA.how_interested_in('D'), usize::MAX);
    }
}
