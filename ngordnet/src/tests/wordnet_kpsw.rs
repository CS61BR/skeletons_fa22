use crate::{handler::NGordNetHandler, tests::new_query};

/// Tests for K != 0, single word
#[test]
fn test_big() {
    let handler = NGordNetHandler::new(
        "data/ngrams/top_14377_words.csv",
        "data/ngrams/total_counts.csv",
        "data/wordnet/synsets.txt",
        "data/wordnet/hyponyms.txt",
    )
    .expect("error loading files");

    assert_eq!(
        handler.respond_synonyms(new_query("dash", 2007, 2007, 0)),
        "bolt, dah, dash, elan, flair, hyphen, panache, sprint, style"
    );

    assert_eq!(
        handler.respond_hyponyms(new_query("dash", 2007, 2007, 0)),
        "bolt, break, dah, dash, elan, fast_break, flair, hyphen, panache, sprint, style"
    );

    assert_eq!(
        handler.respond_synonyms(new_query("dash", 2007, 2007, 1)),
        "style"
    );

    assert_eq!(
        handler.respond_hyponyms(new_query("dash", 2007, 2007, 1)),
        "style"
    );

    assert_eq!(
        handler.respond_synonyms(new_query("dash", 2007, 2007, 3)),
        "bolt, dash, style"
    );

    assert_eq!(
        handler.respond_hyponyms(new_query("dash", 2007, 2007, 3)),
        "bolt, break, style"
    );

    assert_eq!(
        handler.respond_synonyms(new_query("dash", 1700, 1703, 3)),
        "bolt, dash, style"
    );

    assert_eq!(
        handler.respond_hyponyms(new_query("dash", 1700, 1703, 3)),
        "break, dash, style"
    );

    assert_eq!(
        handler.respond_synonyms(new_query("dog", 2007, 2007, 1)),
        "dog"
    );

    assert_eq!(
        handler.respond_hyponyms(new_query("dog", 2007, 2007, 1)),
        "dog"
    );
}
