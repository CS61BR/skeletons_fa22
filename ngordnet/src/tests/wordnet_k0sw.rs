use crate::{handler::NGordNetHandler, tests::new_query};

/// Tests for K = 0, single word
#[test]
fn test_act() {
    let handler = NGordNetHandler::new(
        "data/ngrams/very_short.csv",
        "data/ngrams/total_counts.csv",
        "data/wordnet/synsets16.txt",
        "data/wordnet/hyponyms16.txt",
    )
    .expect("error loading files");

    assert_eq!(
        handler.respond_synonyms(new_query("act", 0, 0, 0)),
        "act, human_action, human_activity"
    );
    assert_eq!(
        handler.respond_hyponyms(new_query("act", 0, 0, 0)),
        "act, action, change, demotion, human_action, human_activity, variation"
    );
}
