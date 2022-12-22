use crate::{handler::NGordNetHandler, tests::new_query};

/// Tests for K != 0, multiple words
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
        handler.respond_hyponyms(new_query("child, animal", 2007, 2007, 0)),
        "baby, kid, monkey, nestling, orphan, suckling, yearling"
    );

    assert_eq!(
        handler.respond_hyponyms(new_query("child, animal", 2007, 2007, 1)),
        "baby"
    );

    assert_eq!(
        handler.respond_hyponyms(new_query("child, animal", 2007, 2007, 3)),
        "baby, kid, monkey"
    );

    assert_eq!(
        handler.respond_hyponyms(new_query("map, function", 1900, 2020, 10)),
        "expansion, function, identity, map, operator, sec, series, sin, transformation, translation"
    );

    assert_eq!(
        handler.respond_hyponyms(new_query("energy, light, sparkle", 1900, 2020, 1)),
        "light"
    );

    assert_eq!(
        handler.respond_hyponyms(new_query("energy, light, beam", 1900, 2020, 3)),
        "beam, ray, shaft"
    );
}
