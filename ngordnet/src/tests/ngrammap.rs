use crate::{handler::NGordNetHandler, ngrammap::NGramMap, Query};

#[test]
fn test_total_count() {
    let ngm = NGramMap::new(
        "./data/ngrams/very_short.csv",
        "./data/ngrams/total_counts.csv",
    )
    .expect("error loading files");

    assert_eq!(ngm.total_count("airport", 2005, 2005), 0.0);
    assert_eq!(ngm.total_count("airport", 2006, 2006), 0.0);
    assert_eq!(ngm.total_count("airport", 2007, 2007), 175702.0);
    assert_eq!(ngm.total_count("airport", 2008, 2008), 173294.0);
    assert_eq!(ngm.total_count("airport", 2005, 2008), 348996.0);
    assert_eq!(ngm.total_count("airport", 2005, 2030), 348996.0);
    assert_eq!(ngm.total_count("airport", 2020, 2030), 0.0);

    assert_eq!(ngm.total_count("request", 2005, 2030), 2816909.0);
    assert_eq!(ngm.total_count("wandered", 2005, 2030), 451106.0);
    assert_eq!(ngm.total_count("cheese", 2005, 2008), 0.0);
}

#[test]
fn test_on_large_file() {
    let ngm = NGramMap::new(
        "./data/ngrams/top_14377_words.csv",
        "./data/ngrams/total_counts.csv",
    )
    .expect("error loading files");

    assert_eq!(ngm.total_count("fish", 1865, 1865), 136497.0);
    assert_eq!(ngm.total_count("fish", 1922, 1922), 444924.0);
    println!(
        "getting total count for fish: {}",
        ngm.total_count("fish", 1850, 1933)
    );
    assert!((ngm.total_count("fish", 1850, 1933) - 24245438.0).abs() < 1e-10);

    let fish_weight = ngm.weight_history("fish", 1850, 1933);
    println!("getting weight for fish: {:?}", fish_weight.data.get(&1865));
    assert!((fish_weight.data.get(&1865).unwrap() - 136497.0 / 2563919231.0).abs() < 1e-7);

    let dog_weight = ngm.weight_history("dog", 1850, 1876);
    println!("getting weight for dog: {:?}", dog_weight.data.get(&1865));
    assert!((dog_weight.data.get(&1865).unwrap() - 75819.0 / 2563919231.0).abs() < 1e-7);

    let both_weight = ngm.summed_weight_history(&vec!["fish", "dog"], 1850, 9001);
    let expected = (136497.0 + 75819.0) / 2563919231.0;
    println!("getting weight for both: {:?}", both_weight.data.get(&1865));
    assert!((both_weight.data.get(&1865).unwrap() - expected).abs() < 1e-7);
}

#[test]
fn test_handler_small() {
    let handler = NGordNetHandler::new(
        "data/ngrams/very_short.csv",
        "data/ngrams/total_counts.csv",
        "data/wordnet/synsets16.txt",
        "data/wordnet/hyponyms16.txt",
    )
    .expect("error loading files");

    // note that query.words is badly formatted - use .trim() to remove extranous spaces
    let query = Query {
        words: "airport, \t \trequest, airport +  \nwandered,wandered+request",
        start_year: 2005,
        end_year: 2007,
        k: 1234567,
    };
    assert_eq!(
        handler.respond_history(query),
        "{\n  \
        \"airport\": [2007, 6.207e-6],\n  \
        \"request\": [2005, 2.428e-5, 2006, 2.447e-5, 2007, 2.464e-5],\n  \
        \"airport + wandered\": [2005, 3.148e-6, 2006, 3.166e-6, 2007, 1.004e-5],\n  \
        \"wandered + request\": [2005, 2.743e-5, 2006, 2.764e-5, 2007, 2.848e-5]\n\
        }"
    );
}
