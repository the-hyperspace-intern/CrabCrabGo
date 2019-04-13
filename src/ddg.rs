use select::document::Document;

use select::predicate::Class;

mod skeleton;
use skeleton::ResultDDG;

pub fn scrap(target: &str) -> Result<Document, Box<std::error::Error>> {
    let url = format!("https://duckduckgo.com/html/?q={}", target);
    let resp = reqwest::get(&url).unwrap();
    assert!(resp.status().is_success());

    let m = Document::from_read(resp)?;

    Ok(m)
}

pub fn vec(doc: Document) {
    let found = doc.find(Class("result__a"));
    found.filter_map(|n| {
        let link = match n.attr("href") {
            Some(l) => l,
            None => "No Link",
        };

        Some(
            ResultDDG {
            title: n.text(),
            link: String::from(link),
            id: n.index(),
            }
        )}
        ).for_each(|x| println!("{:#?}", x));
}  