use crate::expressions::expression_types::Expression;
use crate::expressions::evaluate_expressions::parse_input;
use std::io;

pub mod expressions;

//TODO: Allow different notations other than polish
//TODO: Allow parenthesis[in prog] 
//TODO: Allow different types based on input and operation

fn main() {
    let mut input_buff = String::new();
    let _ = io::stdin().read_line(&mut input_buff);
    let input_parse: Vec<String> = input_buff.trim().split(' ').collect::<Vec<&str>>().iter().map(|&s| s.to_string()).collect();
	let Some(parse_tree) = parse_input(input_parse) else { return };
		print!("{} = ", parse_tree.stringify());
        println!("{}", parse_tree.evaluate());
}
