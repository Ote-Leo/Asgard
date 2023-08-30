pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    match (a, b) {
        ("", b) => b.chars().count(),
        (a, "") => a.chars().count(),
        _ => {
            let mut a_chars = a.chars();
            let mut b_chars = b.chars();

            let a_head = a_chars.next().expect("must be a bug in rust");
            let b_head = b_chars.next().expect("must be a bug in rust");

            let a_tail = a_chars.as_str();
            let b_tail = b_chars.as_str();

            if a_head == b_head {
                levenshtein_distance(a_tail, b_tail)
            } else {
                let cv = levenshtein_distance(a, b_tail);
                let th = levenshtein_distance(a_tail, b);
                let pb = levenshtein_distance(a_tail, b_tail);
                1 + [cv, th, pb].iter().min().unwrap()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_cases_have_no_distance() {
        let a = "";
        let b = "";
        let output = 0;
        assert_eq!(levenshtein_distance(a, b), output);
    }

    #[test]
    fn match_test() {
        let a = "HC SVNT DRACONES";
        let b = "HC SVNT DRACONES";
        let output = 0;
        assert_eq!(levenshtein_distance(a, b), output);
    }

    #[test]
    fn basic_single_difference_test() {
        let inputs = [("A", ""), ("", "Z"), ("A", "Z")];

        for (a, b) in inputs {
            assert_eq!(levenshtein_distance(a, b), 1);
        }
    }

    #[test]
    fn basic_difference_test() {
        let inputs = [
            ("Java", "JavaScript"),
            ("atomic", "atom"),
            ("flaw", "lawn"),
            ("object", "inject"),
            ("gattaca", "tataa"),
            ("attaca", "tataa"),
            ("bullfrog", "frogger"),
            ("levenshtein", "levenshtein"),
        ];
        let outputs = [6, 2, 2, 2, 3, 3, 7, 0];

        for ((a, b), output) in inputs.iter().zip(outputs) {
            assert_eq!(levenshtein_distance(a, b), output);
        }
    }


    #[test]
    fn distance_is_case_sensitive() {
        assert_eq!(levenshtein_distance("DwAyNE", "DUANE"), 2);
        assert_eq!(levenshtein_distance("dwayne", "DuAnE"), 5);
    }

    #[test]
    fn distance_is_the_same_in_either_directions() {
        let a = "aarrgh";
        let b = "aargh";
        let output = 1;
        assert_eq!(levenshtein_distance(a, b), output);
        assert_eq!(levenshtein_distance(b, a), output);
    }

    #[test]
    fn should_work_test() {
        assert_eq!(levenshtein_distance("sitting", "kitten"), 3);
        assert_eq!(levenshtein_distance("gumbo", "gambol"), 2);
        assert_eq!(levenshtein_distance("saturday", "sunday"), 3);
        assert_eq!(levenshtein_distance("a", "b"), 1);
        assert_eq!(levenshtein_distance("ab", "ac"), 1);
        assert_eq!(levenshtein_distance("ac", "bc"), 1);
        assert_eq!(levenshtein_distance("abc", "axc"), 1);
        assert_eq!(levenshtein_distance("xabxcdxxefxgx", "1ab2cd34ef5g6"), 6);
        assert_eq!(levenshtein_distance("xabxcdxxefxgx", "abcdefg"), 6);
        assert_eq!(levenshtein_distance("javawasneat", "scalaisgreat"), 7);
        assert_eq!(levenshtein_distance("example", "samples"), 3);
        assert_eq!(levenshtein_distance("sturgeon", "urgently"), 6);
        assert_eq!(levenshtein_distance("levenshtein", "frankenstein"), 6);
        assert_eq!(levenshtein_distance("distance", "difference"), 5);
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("Tier", "Tor"), 2);
    }

    #[test]
    fn unicode_glyphs_are_counted_as_one() {
        assert_eq!(levenshtein_distance("😄", ""), 1);
        assert_eq!(levenshtein_distance("", "😄"), 1);
        assert_eq!(levenshtein_distance("😄", "😦"), 1);
        assert_eq!(levenshtein_distance("😘", "😘"), 0);
    }
}
