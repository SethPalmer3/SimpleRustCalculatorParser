use crate::expressions::expression_actions::new_operation;
use crate::Expression;

use super::expression_actions::{new_parenthesis, new_end_parenthesis};

fn string_operation(operator: String) -> Box<Expression>{
    let i = operator.parse::<i32>();
	if i.is_err(){
		match operator.as_str() {
			"+" => {
                return new_operation(Box::new(|a,b|a+b), "+".to_string());
			},
            "-" => {
                return new_operation(Box::new(|a,b|a-b), "-".to_string());
            },
            "x" => {
                return new_operation(Box::new(|a,b|a*b), "*".to_string());
            },
            "%" => {
                return new_operation(Box::new(|a,b|a/b), "/".to_string());
            },
            "(" => {
                return new_parenthesis(None);
            },
            ")" => {
                return new_end_parenthesis();
            }
			_ => Box::new(Expression::NULL),
		}
	}
	else {
		return Box::new(Expression::NULL)
	}
}

/// Takes a new expression node and adds to stack while flattening the "stack"
/// # Arguments
/// * 'node_vec' - A vector that represents a stack for expression nodes
/// * 'new_node' - A new expression node to be added to the stack
fn add_expr(node_vec: &mut Vec<Box<Expression>>, new_node: Box<Expression>){
	// Move all completed values into non-completed values
	if let Some(mut last_on_stack) = node_vec.pop() { // Assumes Polish notation
        if !last_on_stack.add_child_node(new_node){
            panic!("Illegal adding of expression, possible syntax error");
        }else{
            node_vec.push(last_on_stack);
        }
	}else{
		node_vec.push(new_node);
	}
}

/// Takes a vector of strings and returns a final expression representation
/// # Arguments
/// * 'expr' - A vector of strings to be evaluated
pub fn parse_input(expr: Vec<String>) -> Option<Box<Expression>> {
	let mut node_stack: Vec<Box<Expression>> = Vec::new();
	for s in expr {
		let integer = s.parse::<i32>();
		let new_node: Box<Expression>;
		if integer.is_err() {	// Handle operators
			new_node = string_operation(s);
		}else {
			new_node = Box::new(Expression::Constant(integer.unwrap()));
		}
		add_expr(&mut node_stack, new_node);
	}
	return node_stack.pop();
}
