use std::io::BufRead;

pub struct Dummy {
    quotes: Vec<String>,
}

impl Dummy {
    pub fn new(file: &str) -> Result<Self, std::io::Error> {
        let file = std::fs::File::open(file)?;
        let file = std::io::BufReader::new(file);

        let mut quotes = Vec::new();
        for line in file.lines() {
            let line = line?;
            let mut split_by_commas = line.split(", ");

            let Some(name) = split_by_commas.next() else {continue;};
            let rest: Vec<&str> = split_by_commas.collect();
            let quote = rest.join(", ");
            quotes.push(format!("\"{}\" - {}", quote, name));
        }
        Ok(Self { quotes })
    }

    pub fn quote(&self) -> &str {
        let i = rand::random::<usize>() % self.quotes.len();
        &self.quotes[i]
    }
}
