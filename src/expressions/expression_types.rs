pub enum Expression {
	Operation {
		l: Box<Expression>,
		r: Box<Expression>,
		op: Box<dyn Fn(i32, i32) -> i32>,
		symbol: String,
	},
	Constant(i32),
    Parentheses(Box<Expression>),
    EndParenthesis,
	NULL,
}

impl Expression {
    pub(crate) fn evaluate(&self) -> i32 {
		match self {
			Expression::Operation { l, r, op, symbol: _ } => {
				let left = l.evaluate();
				let right = r.evaluate();
				return (op)(left, right);
			}
			Expression::Constant(l) => *l,
            Expression::Parentheses(e) => e.evaluate(),
			_ => panic!("Error when handling expression"),
		}
	}
	pub(crate) fn stringify(&self) -> String{
		match self {
			Expression::Operation { l, r, op: _, symbol }=>{
				let mut stringy: String = l.stringify();
				stringy.push(' ');
				stringy.push_str(symbol);
				stringy.push(' ');
				stringy.push_str(r.stringify().as_str());
				return stringy;
			}
            Expression::Constant(i) => i.to_string(),
            Expression::Parentheses(e) => {
                let mut s: String = "(".to_string();
                s.push_str(e.stringify().as_str());
                s.push(')');
                return s;
            },
            _ => String::from("NULL"),
		}
	}
	pub(crate) fn is_complete_expr(&self) -> bool {
		match self {
			Expression::Operation { l, r, op: _, symbol: _ } => {
				return l.is_complete_expr() && r.is_complete_expr();
			}
			Expression::Constant(_) => true,
            Expression::Parentheses(e) => {
                return e.is_complete_expr();
            },
            Expression::EndParenthesis => true,
			_ => false,

		}
	}
    pub(crate) fn is_null(&self) -> bool {
        match self {
            Expression::NULL => true,
            Expression::EndParenthesis => true,
            _ => false,
        }
    }
	pub(crate) fn add_child_node(&mut self, node: Box<Expression>) -> bool{
		match self {
			Expression::Operation { l, r, op: _, symbol: _ }=>{
				if l.is_null() {
					*l = node;
                    return true;
				}else if !l.is_complete_expr(){
                    l.add_child_node(node);
                    return true;
				}else if r.is_null() {
					*r = node;
                    return true;
                }else if !r.is_complete_expr(){
                    r.add_child_node(node);
                    return true;
                }
				return false;
			}
            Expression::Constant(_) => false,
            Expression::Parentheses(e) => {
                if let Expression::EndParenthesis = *node{
                    if !e.is_complete_expr() {
                        panic!("Parenthesis closed before completing inner expression");
                    }else{
                        return true;
                    }
                }
                if e.is_null(){
                    *e = node;
                    return true;
                }else if !e.is_complete_expr(){
                    e.add_child_node(node);
                    return true;
                }
                return false;
            },
            _ => false,
		}
	}
}
