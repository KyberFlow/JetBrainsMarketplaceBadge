// MARKETPLACE_ID need to be set 

use std::env;

fn main() {
    let key = "MARKETPLACE_ID";
    let val = env::var(key).unwrap();
    println!("{}: {:?}", key, val); 
}
