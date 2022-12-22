use crate::{ngrammap::NGramMap, timeseries::TimeSeries, Query, dummy::Dummy};
use std::fmt::Write;

/// keeps server state necessary to respond to requests
pub struct NGordNetHandler {
    dummy: Dummy
}

impl NGordNetHandler {
    pub fn new(
        words_file: &str,
        counts_file: &str,
        synset_file: &str,
        hyponym_file: &str,
    ) -> Result<Self, std::io::Error> {
        Ok(Self {
            dummy: Dummy::new("quotes.txt")?
        })
    }

    pub fn respond_history(&self, query: Query) -> String {
        format!("history query: {:?}\n{}", query, self.dummy.quote())
    }

    pub fn respond_synonyms(&self, query: Query) -> String {
        format!("synonyms query: {:?}\n{}", query, self.dummy.quote())
    }

    pub fn respond_hyponyms(&self, query: Query) -> String {
        format!("hyponyms query: {:?}\n{}", query, self.dummy.quote())
    }
}

/// Takes a list of words and their corresponding TimeSeries and
/// outputs a JSON string, from which the frontent can create a graph
fn json_object_string(mapping: &Vec<(String, TimeSeries)>) -> String {
    let mut json = String::from("{");
    for (k, v) in mapping {
        json += "\n  \"";
        json += &json_escape(k);
        json += "\": [";
        for (k, v) in &v.data {
            write!(&mut json, "{}, {:.3e}, ", k, v).unwrap();
        }
        if !v.data.is_empty() {
            json.pop();
            json.pop();
        }
        json += "],";
    }
    if !mapping.is_empty() {
        json.pop();
    }
    json += "\n}";
    json
}

/// Escapes a string for use in JSON
fn json_escape(src: &str) -> String {
    let mut escaped = String::with_capacity(src.len());
    let mut utf16_buf = [0u16; 2];
    for c in src.chars() {
        match c {
            '\x08' => escaped += "\\b",
            '\x0c' => escaped += "\\f",
            '\n' => escaped += "\\n",
            '\r' => escaped += "\\r",
            '\t' => escaped += "\\t",
            '"' => escaped += "\\\"",
            '\\' => escaped += "\\",
            ' ' => escaped += " ",
            c if c.is_ascii_graphic() => escaped.push(c),
            c => {
                let encoded = c.encode_utf16(&mut utf16_buf);
                for utf16 in encoded {
                    write!(&mut escaped, "\\u{:04X}", utf16).unwrap();
                }
            }
        }
    }
    escaped
}
