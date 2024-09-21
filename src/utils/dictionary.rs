use std::vec;

use regex::Regex;
use lazy_static::lazy_static;

#[derive(Clone, Debug)]
pub struct TkT {
    pub reg: Regex,
    pub id: String,
}
#[allow(dead_code)]
pub struct Keyword{
	pub comparator : Vec<String>,
	pub id : String
}

lazy_static!{
	pub static ref DICTIONARY: Vec<TkT> = vec![
			TkT { // String
				reg: Regex::new(r#"[\"'](?:\\.|[^\"\\])*[\"']"#).unwrap(),
				id: "STRING_TK".to_string(),
			},
			TkT { // Number
				reg: Regex::new(r#"[-]?\d+(\.\d+)?([eE][-]?\d+)?"#).unwrap(),
				id: "NUMBER_TK".to_string(),
			},
			TkT { // Access Token
				reg: Regex::new(r#"(\??\.)"#).unwrap(),
				id: "ACCESS_TK".to_string()
			},
			TkT { // Binary Operator
				reg: Regex::new(r#"(>>>|\?\?|[!=><]=(=?)|[*&|^<>]{1,2}|[+/%])"#).unwrap(),
				id: "BINOP_TK".to_string(),
			},
			TkT { // Assign
				reg: Regex::new(r#"(>>>=|[+/*%-&|^-]?=|[<>&|?*]{2}=)"#).unwrap(),
				id: "ASSIGN_TK".to_string(),
			},
			TkT { // Identifier
				reg: Regex::new(r#"[a-zA-Z_][a-zA-Z0-9_]*"#).unwrap(),
				id: "IDENTIFIER_TK".to_string(),
			},
			TkT { // Opar
				reg: Regex::new(r#"\("#).unwrap(),
				id: "OPAR_TK".to_string(),
			},
			TkT { // Cpar
				reg: Regex::new(r#"\)"#).unwrap(),
				id: "CPAR_TK".to_string(),
			},
			TkT { // Obra
				reg: Regex::new(r#"\["#).unwrap(),
				id: "OBRA_TK".to_string(),
			},
			TkT { // Cbra
				reg: Regex::new(r#"\]"#).unwrap(),
				id: "CBRA_TK".to_string(),
			},
			TkT { // Obracs
				reg: Regex::new(r#"\{"#).unwrap(),
				id: "OBRACS_TK".to_string(),
			},
			TkT { // Cbracs
				reg: Regex::new(r#"\}"#).unwrap(),
				id: "CBRACS_TK".to_string(),
			},
			TkT { // Comma
				reg: Regex::new(r#"\,"#).unwrap(),
				id: "COMMA_TK".to_string(),
			},
			
			TkT { // EOF
				reg: Regex::new(r#"\z"#).unwrap(),
				id: "EOF_TK".to_string(),
			},
		];
	
		pub static ref commentReg : Regex = Regex::new(r#"/\/\*[^*]*\*+([^/*][^*]*\*+)*\//g"#).unwrap();
	
		pub static ref master : Regex =	{
			let pattern = DICTIONARY.iter()
			.map(|tkt| tkt.reg.as_str())
			.collect::<Vec<&str>>()
			.join("|");
			Regex::new(&pattern).unwrap()
		};

		pub static ref KEYWORDS : Vec<Keyword> = vec![
			Keyword{
				comparator: vec!["let".to_string(), "const".to_string()],
				id: "DECLAR_TK".to_string()
			},
			Keyword {
				comparator: vec!["let".to_string(), "const".to_string()],
				id: "DECLAR_TK".to_string()
			}
		];
}


