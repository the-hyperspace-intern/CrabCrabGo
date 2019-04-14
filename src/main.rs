extern crate reqwest;
extern crate select;
extern crate colored;
extern crate open;


mod ddg;
mod ui;
mod skeleton;


fn main() {
    let data = ddg::scrap("x");
    let result_data = match data {
        Ok(doc) => {
            ddg::vec(doc)
        },
        Err(error) => panic!("{:#?}", error),
    };

    ui::show(&result_data);
    let i = result_data.first().unwrap();
    ddg::open(i);

}




