use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let article: Article = Article {
        article: String::from("How to work with json in rust"),
        author: String::from("Muktar Sadiq"),
        paragraph: vec![
            Paragraph {
                name: String::from("First sentence of the paragraph"),
            },
            Paragraph {
                name: String::from("Body of the paragraph"),
            },
            Paragraph {
                name: String::from("End of the paragraph"),
            },
        ],
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("the json is {}", json);
}
