extern crate reqwest;
extern crate select;


mod ddg;

fn main() {
    let data = ddg::scrap("ginkoe github");
    match data {
        Ok(doc) => {
            ddg::vec(doc);
        },
        Err(error) => {println!("Error Occured : {:#?}", error)}
    }
}




