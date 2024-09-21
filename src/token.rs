
#[path = "utils/dictionary.rs"]
mod dictionary;
use std::vec;
use dictionary::{master, TkT, DICTIONARY};

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Token{
	value: String,
	id: String
}

pub fn sanitize(raw : String) -> Vec<Vec<String>> {

	let captures: Vec<Vec<String>> = raw
    .replace("\r", "")
    .replace("\n", "")
    .replace("\r\n", "")
    .split(';')
    .map(|x| {
        master.find_iter(x.trim()) // Encuentra todas las coincidencias
            .map(|mat| mat.as_str().to_string()) // Convierte cada coincidencia en una cadena
            .into_iter()
			.filter(|p| p.len() > 0)
			.collect::<Vec<String>>() // Recolecta todas las coincidencias en un vector
    })
	.into_iter()
	.filter(|p| p.len() > 0)
    .collect();


	return captures;
}

fn get_tk(part : String) -> Token{
	let founds : Vec<TkT> = DICTIONARY
	.iter()
    .filter(|item| item.reg.is_match(&part))
	.cloned()
    .collect();
	let tk = founds.get(0).cloned().unwrap();

	return Token{
		value: part,
		id: tk.id
	};
}

pub fn tokenize(caps : Vec<Vec<String>>) -> Vec<Vec<Token>>{
	let mut ret_vec : Vec<Vec<Token>> = Vec::new();
	for main in caps{
		let mut aux_vec : Vec<Token> = Vec::new();
		for part in main{
			aux_vec.push(get_tk(part));
		}
		ret_vec.push(aux_vec);
	}

	ret_vec.push(vec![Token{value:"".to_string(), id: "EOF_TK".to_string()}]);

	return ret_vec;
}