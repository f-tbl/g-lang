use std::env;
use std::fs::File;
use std::io::Read;

use token::{sanitize, tokenize};


mod token;

fn main(){
	let args: Vec<String> = env::args().collect();
	if(args.len() as i32) < 2 {panic!("ERROR")};
	let filename = &args[1];

	let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
	let _= file.read_to_string(&mut contents);
	
	let results = sanitize(contents);

	dbg!(tokenize(results));

}