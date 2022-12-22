mod handler;
mod ngrammap;
#[cfg(test)]
mod tests;
mod timeseries;
mod dummy;

use handler::NGordNetHandler;
use rocket::{fs::FileServer, State};

#[macro_use]
extern crate rocket;

#[derive(FromForm, Debug, Clone)]
pub struct Query<'r> {
    words: &'r str,
    #[field(name = "start-year")]
    start_year: usize,
    #[field(name = "end-year")]
    end_year: usize,
    k: usize,
}

#[get("/history?<query..>")]
fn history(query: Query, handler: &State<NGordNetHandler>) -> String {
    handler.respond_history(query)
}

#[get("/synonyms?<query..>")]
fn synonyms(query: Query, handler: &State<NGordNetHandler>) -> String {
    handler.respond_synonyms(query)
}

#[get("/hyponyms?<query..>")]
fn hyponyms(query: Query, handler: &State<NGordNetHandler>) -> String {
    handler.respond_hyponyms(query)
}

#[launch]
fn rocket() -> _ {
    let time = std::time::Instant::now();
    let handler = NGordNetHandler::new(
        "./data/ngrams/top_49887_words.csv",
        "./data/ngrams/total_counts.csv",
        "./data/wordnet/synsets.txt",
        "./data/wordnet/hyponyms.txt",
    )
    .expect("Error when creating handler!");
    let duration = time.elapsed().as_secs_f64();
    println!("Finished constructing handler in {duration} seconds");

    rocket::build()
        .mount("/", FileServer::from("public"))
        .mount("/", routes![history, synonyms, hyponyms])
        .manage(handler)
}
