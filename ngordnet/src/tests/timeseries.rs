use crate::timeseries::TimeSeries;

#[test]
fn from_spec() {
    let mut cat_population = TimeSeries::new();
    cat_population.data.insert(1991, 0.0);
    cat_population.data.insert(1992, 100.0);
    cat_population.data.insert(1994, 200.0);

    let mut dog_population = TimeSeries::new();
    dog_population.data.insert(1994, 400.0);
    dog_population.data.insert(1995, 500.0);

    let mut total_population = cat_population.clone();
    total_population += &dog_population;

    assert_eq!(
        format!("{total_population:?}"),
        "TimeSeries { data: {1991: 0.0, 1992: 100.0, 1994: 600.0, 1995: 500.0} }"
    )
}
