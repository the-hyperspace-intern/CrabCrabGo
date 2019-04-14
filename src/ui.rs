use crate::skeleton::ResultDDG;
use colored::*;

pub fn show(list: &Vec<ResultDDG>) {
    for result in list {
        println!("{}{}{} {}\n", "[".green(), result.id.to_string().green(), "]".green(), result.title.bold());
        println!("{}", result.desc.italic());
    };
}