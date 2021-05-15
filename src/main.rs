mod cli;
mod dep_checker;
mod errors;
mod fs_helper;
use crate::dep_checker::check_deps;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    match check_deps() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err)
        }
    };
    let elapsed = now.elapsed();
    println!("Executed in: {:?}", elapsed);
}
