use crate::Query;

mod ngrammap;
mod timeseries;
mod wordnet_k0mw;
mod wordnet_k0sw;
mod wordnet_kpmw;
mod wordnet_kpsw;

fn new_query(words: &str, start_year: usize, end_year: usize, k: usize) -> Query {
    Query {
        words,
        start_year,
        end_year,
        k,
    }
}
