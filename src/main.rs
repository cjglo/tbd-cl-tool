use std::env;
use std::process;

fn main() {

    let args: Vec<String> = env::args().collect();

    println!("Sucess!");
    
    process::exit(1);

}
