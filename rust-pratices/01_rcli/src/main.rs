use std::fs;

use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
struct Book {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Author")]
    author: String,
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Price")]
    price: f32,
}

impl Book {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}


fn main() -> anyhow::Result<()> {
    let file = fs::File::open("./assets/book.csv")?;
    let mut reader = csv::Reader::from_reader(file);
    for record in reader.deserialize() {
        let line: Book = record?;
        println!("{}", line.to_json());
    }

    Ok(())
}
