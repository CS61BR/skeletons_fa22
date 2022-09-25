# Differences from the Java version


## Error handling in Percolation

In Java, out of bounds indices were handled by throwing an `IndexOutOfBoundsException`. The analogous thing to do in Rust is to return a `Result`, like so:
```
fn open(&mut self, row: usize, col: usize) -> Result<(), &'static str> {
    ...
}
```
If `row` or `col` is out of bounds, `open` can return something like `Err("out of bounds")`. If everything goes well, it can return `Ok(())`. 


## Dependency Injection
Both the Rust and Java versions do dependency injection, but slightly differently:
 - the Java version uses a class `PercolationFactory`, and subclasses both `PercolationFactory` and `Percolation` to provide a different implementation of `Percolation`
 - the Rust version uses a trait `Percolatable`, and uses a closure `percolation_producer` in place of the `PercolationFactory`

To get a `Percolatable` object from `percolation_producer`, you can simply call it:
```
let mut p = percolation_producer();
```

## Counts in Percolation Statistics

For the purposes of graph generation, `calculate_stats` also returns a `counts` array. This array tracks how many percolation instances percolated at each number of open sites. For example, this array
```
[0, 0, 0, 0, 1, 4, 2, 5, 0, 1, 0, 0]
```
means that out of 13 total trials, 1 trial percolated after 4 sites were opened, 4 trials percolated after 5 sites were opened, 2 trials percolated after 6 sites were opened, and so on.

## Miscellaneous

 - `Percolation` and `PercolationStats` are expected to handle non-square grids
 - no `stddev` calculator is provided; that will need to be calculated in `calculate_stats`.
 - `Percolation` will need to implement a few extra methods (`width()` and `height()`)

