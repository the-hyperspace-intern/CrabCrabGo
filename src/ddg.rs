use select::document::Document;

use urlparse::urlparse;
use urlparse::GetQuery;

use select::predicate::Class;

use crate::skeleton::ResultDDG;

pub fn scrap(target: &str) -> Result<Document, Box<std::error::Error>> {
    let url = format!("https://duckduckgo.com/html/?q={}", target);
    let resp = reqwest::get(&url).unwrap();
    assert!(resp.status().is_success());

    let m = Document::from_read(resp)?;

    Ok(m)
}

pub fn vec(doc: Document) -> Vec<ResultDDG> {
    let mut i: usize = 0;
    let a_result = doc.find(Class("result__a"));

    let expected_size = a_result.size_hint().0;

    let mut feedback = Vec::with_capacity(expected_size);
    

    a_result.filter_map(|n| {
        let a_descsnipper = doc.find(Class("result__snippet")).nth(i).unwrap().text();
        i += 1;
        
        let mut link = String::new();
        match n.attr("href") {

            Some(l) => {
                let urlparsed = urlparse(l);
                let query = urlparsed.get_parsed_query().unwrap();
                let uri = query.get("uddg").unwrap().first().unwrap().to_owned();
                link.push_str(&uri);
            },
            None => println!("No Link"),
        };

        Some(
            ResultDDG {
            title: n.text(),
            link: String::from(link),
            desc: String::from(a_descsnipper),
            id: i,
            }
        )
        }
        ).for_each(|x| feedback.push(x));

    feedback
}

pub fn open(target: &ResultDDG) {
    if open::that(&target.link).is_ok() {
        println!("Look at your browser !")
    } else {
        println!("An Error Has Occured")
    }
}