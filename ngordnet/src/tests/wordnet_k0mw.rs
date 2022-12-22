use crate::{handler::NGordNetHandler, tests::new_query};

/// Tests for K = 0, multiple words
/// using small data files
#[test]
fn test_small() {
    let handler = NGordNetHandler::new(
        "data/ngrams/very_short.csv",
        "data/ngrams/total_counts.csv",
        "data/wordnet/synsets16.txt",
        "data/wordnet/hyponyms16.txt",
    )
    .expect("error loading files");

    assert_eq!(
        handler.respond_synonyms(new_query("modification, change", 0, 0, 0)),
        "alteration, change, modification"
    );

    assert_eq!(
        handler.respond_hyponyms(new_query("modification, change", 0, 0, 0)),
        "alteration, change, increase, jump, leap, modification, saltation, transition"
    );

    assert_eq!(
        handler.respond_synonyms(new_query("occurrence, change", 0, 0, 0)),
        ""
    );
    assert_eq!(
        handler.respond_hyponyms(new_query("occurrence, change", 0, 0, 0)),
        "alteration, change, increase, jump, leap, modification, saltation, transition"
    );
}

/// Tests for K = 0, multiple words
/// using large data files
#[test]
fn test_large() {
    let handler = NGordNetHandler::new(
        "data/ngrams/very_short.csv",
        "data/ngrams/total_counts.csv",
        "data/wordnet/synsets.txt",
        "data/wordnet/hyponyms.txt",
    )
    .expect("error loading files");

    assert_eq!(
        handler.respond_hyponyms(new_query("bowl, gallery", 0, 0, 0)),
        "amphitheater, amphitheatre"
    );

    assert_eq!(
        handler.respond_hyponyms(new_query("female, animal", 0, 0, 0)),
        "amazon, bird, cat, chick, dam, demoiselle, female, female_mammal, filly, hag, hen, nanny, nymph, siren"
    );

    assert_eq!(
        handler.respond_hyponyms(new_query("female, leader", 0, 0, 0)),
        "crown_princess, marchioness, materfamilias, matriarch, mayoress, mistress, vicereine, viscountess"
    );

    assert_eq!(
        handler.respond_hyponyms(new_query("energy, light, sparkle", 0, 0, 0)),
        "light, scintillation, spark, sparkle, twinkle"
    );
}
