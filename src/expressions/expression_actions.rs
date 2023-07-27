use crate::Expression;


pub fn new_operation(op: Box<dyn Fn(i32, i32) -> i32>, symbol: String) -> Box<Expression>{
    let new_op: Expression = Expression::Operation { l: Box::new(Expression::NULL),
                                                     r: Box::new(Expression::NULL),
                                                     op,
                                                    symbol, };
    return Box::new(new_op);
}

pub fn new_parenthesis(expr: Option<Box<Expression>>) -> Box<Expression>{
    let input_data: Box<Expression>;
    if let Some(data) = expr {
        input_data = Box::new(*data);
    }else {
        input_data = Box::new(Expression::NULL);
    }
    let new_paren = Expression::Parentheses(input_data); 
    return Box::new(new_paren);
}

pub fn new_end_parenthesis() -> Box<Expression>{
    let new_paren = Expression::EndParenthesis; 
    return Box::new(new_paren);
}
