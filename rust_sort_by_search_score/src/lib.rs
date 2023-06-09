use std::cmp::Ordering;

struct SourceScore {
    score: f32,
    source: String,
}

#[inline]
fn calculate_score(search_term: &String, source: &String) -> f32 {
    if source.starts_with(search_term) {
        1f32 + (search_term.len() as f32 / source.len() as f32)
    } else if source.contains(search_term) {
        search_term.len() as f32 / source.len() as f32
    } else {
        -1f32
    }
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
            score: calculate_score(search_term, source),
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
}
