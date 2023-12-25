use std::cmp::Ordering;
use std::collections::HashSet;

struct SourceScore {
    score: f32,
    source: String,
}

#[inline]
fn calculate_substring_score(search_term: &String, source: &String) -> f32 {
    if source.starts_with(search_term) {
        1f32 + (search_term.len() as f32 / source.len() as f32)
    } else if source.contains(search_term) {
        search_term.len() as f32 / source.len() as f32
    } else {
        -1f32
    }
}

#[inline]
fn calculate_jaccard_index(search_term: &String, source: &String) -> f32 {
    if search_term.is_empty() || source.is_empty() {
        return 0f32;
    }

    let mut search_term_chars: HashSet<char> = HashSet::new();
    search_term.chars().for_each(|c| {
        search_term_chars.insert(c);
    });

    let mut intersection: HashSet<char> = HashSet::new();

    let mut source_chars: HashSet<char> = HashSet::new();
    source.chars().for_each(|c| {
        source_chars.insert(c);
        if search_term_chars.contains(&c) {
            intersection.insert(c);
        }
    });

    intersection.len() as f32
        / (search_term_chars.len() as f32 + source_chars.len() as f32 - intersection.len() as f32)
}

#[inline]
fn calculate_levenshtein_distance(search_term: &String, source: &String) -> usize {
    let st_length = search_term.len();
    let ss_length = source.len();

    if st_length == 0 {
        return ss_length;
    }

    if ss_length == 0 {
        return st_length;
    }

    let st = search_term.to_lowercase();
    let ss = source.to_lowercase();

    println!("testing: {} and {}", st, ss);
    if st == ss {
        return 0;
    }

    let mut matrix: Vec<usize> = (0..=(st_length + 1)).collect();

    let search_term_chars: Vec<char> = st.chars().collect();
    let source_chars: Vec<char> = ss.chars().collect();

    for j in 1..(ss_length + 1) {
        let mut previous = matrix[0];
        matrix[0] = j;

        for i in 1..(st_length + 1) {
            let current = matrix[i];

            if search_term_chars[i - 1] == source_chars[j - 1] {
                matrix[i] = previous;
            } else {
                matrix[i] = std::cmp::min(previous, std::cmp::min(matrix[i], matrix[i - 1])) + 1;
            }

            previous = current;
        }
    }

    matrix[st_length]
}

#[inline]
fn compare_floats(left: &f32, right: &f32) -> Ordering {
    if left > right {
        Ordering::Greater
    } else if left < right {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

/// Sorts the provided source by the relevance of the provided search term.
/// #### Arguments
/// * `search_term` - the term (a non-empty string) to search in the provided source of strings
/// * `source` - the source of strings to search in and then sort by the relevant score, assuming that all strings are non-empty
pub fn sort_by_ranking_score(search_term: &String, source: Vec<String>) -> Vec<String> {
    // calculates ranking score
    let mut source_scores: Vec<SourceScore> = source
        .iter()
        .map(|source| SourceScore {
            score: calculate_substring_score(search_term, source),
            source: source.to_string(),
        })
        .collect();

    // sorts (in-place) in descending order by using the item b as the left argument
    source_scores.sort_by(|a, b| compare_floats(&b.score, &a.score));

    // return the source strings only
    source_scores
        .iter()
        .map(|source_score| source_score.source.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_empty_vector_when_empty_source_is_provided() {
        let search_term = String::from("berlin");
        let source: Vec<String> = vec![];

        let sorted = sort_by_ranking_score(&search_term, source);
        let expected: Vec<String> = vec![];

        assert_eq!(sorted, expected);
    }

    #[test]
    fn it_returns_same_sorting_order_when_source_does_not_contain_provided_search_term() {
        let search_term = String::from("berlin");
        let source: Vec<String> = vec![
            String::from("washington"),
            String::from("london"),
            String::from("chicago"),
            String::from("rio"),
            String::from("moscow"),
            String::from("mumbai"),
            String::from("tokyo"),
        ];

        let sorted = sort_by_ranking_score(&search_term, source);
        let expected: Vec<String> = vec![
            String::from("washington"),
            String::from("london"),
            String::from("chicago"),
            String::from("rio"),
            String::from("moscow"),
            String::from("mumbai"),
            String::from("tokyo"),
        ];

        assert_eq!(sorted, expected);
    }

    #[test]
    fn it_returns_sorted_source_when_the_source_contains_the_search_term() {
        let search_term = String::from("berlin");
        let source = vec![
            String::from("berlino"),
            String::from("mo-berlin"),
            String::from("potsdam"),
            String::from("berlin wannsee"),
            String::from("berlin"),
            String::from("weder"),
            String::from("alt berlin"),
        ];

        let sorted = sort_by_ranking_score(&search_term, source);

        let expected = vec![
            String::from("berlin"),
            String::from("berlino"),
            String::from("berlin wannsee"),
            String::from("mo-berlin"),
            String::from("alt berlin"),
            String::from("potsdam"),
            String::from("weder"),
        ];

        assert_eq!(sorted, expected)
    }

    #[test]
    fn it_correctly_calculates_jaccard_index_of_given_strings() {
        let test_cases: Vec<(String, String, f32)> = vec![
            (
                String::from("berlin"),
                String::from("berlino"),
                0.85714287f32,
            ),
            (String::from("berlin"), String::from("berlin"), 1f32),
            (String::from("berlin"), String::from("berl"), 0.6666667f32),
            (String::from("berlin"), String::from("ber"), 0.5f32),
            (String::from("berlin"), String::from("be"), 0.33333333f32),
            (String::from("berlin"), String::from("b"), 0.16666667f32),
            (
                String::from("berlin"),
                String::from("berlins"),
                0.85714287f32,
            ),
            (String::from("hello"), String::from("hello"), 1f32),
            (String::from("hello"), String::from("helo"), 1f32),
            (String::from("hello"), String::from("world"), 0.2857143f32),
            (String::from("pumpkin"), String::from("world"), 0f32),
            (
                String::from("pumpkin"),
                String::from("pumpin"),
                0.8333333f32,
            ),
            (String::from("pumpkin"), String::from("pumpkin"), 1f32),
        ];

        test_cases
            .iter()
            .for_each(|(search_term, source, expected)| {
                let score = calculate_jaccard_index(search_term, source);
                assert_eq!(score, *expected);
            });
    }

    #[test]
    fn it_correctly_calculates_levenshtein_distance_of_given_strings() {
        let test_cases: Vec<(String, String, usize)> = vec![
            (String::from("berlin"), String::from("berlino"), 1),
            (String::from("berlin"), String::from("berlin"), 0),
            (String::from("berlin"), String::from("berl"), 2),
            (String::from("berlin"), String::from("ber"), 3),
            (String::from("berlin"), String::from("be"), 4),
            (String::from("berlin"), String::from("b"), 5),
            (String::from("berlin"), String::from("berlins"), 1),
            (String::from("hello"), String::from("hello"), 0),
            (String::from("hello"), String::from("helo"), 1),
            (String::from("hello"), String::from("world"), 4),
            (String::from("pumpkin"), String::from("world"), 7),
            (String::from("pumpkin"), String::from("pumpin"), 1),
            (String::from("pumpkin"), String::from("pumpkin"), 0),
        ];

        test_cases
            .iter()
            .for_each(|(search_term, source, expected)| {
                let score = calculate_levenshtein_distance(search_term, source);
                assert_eq!(score, *expected);
            });
    }
}
