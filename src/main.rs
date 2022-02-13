use std::string::String;

trait Expression {
    fn compute(&self) -> isize;
    fn pretty_print(&self) -> String;
    fn save(&self) -> String;
}

struct Const {
    value: isize,
}

impl Expression for Const {
    fn compute(&self) -> isize {
        self.value
    }
    fn pretty_print(&self) -> String {
        String::new() 
        + "(" 
        + self.value.to_string().as_str() 
        + ")"
    }
    fn save(&self) -> String {
        String::new() + "C " + self.value.to_string().as_str() + " "
    }
}

struct BinOp {
    operator: char,
    lhs: Box<dyn Expression>,
    rhs: Box<dyn Expression>,
}

impl Expression for BinOp {
    fn compute(&self) -> isize {
        let lhs = self.lhs.compute();
        let rhs = self.rhs.compute();
        match self.operator {
            '+' => lhs + rhs,
            '-' => lhs - rhs,
            '*' => lhs * rhs,
            '/' => lhs / rhs,
            _ => panic!(),
        }
    }
    fn pretty_print(&self) -> String {
        String::new() 
        + "(" 
        + self.lhs.pretty_print().as_str() 
        + self.operator.to_string().as_str() 
        + self.rhs.pretty_print().as_str() 
        + ")"
    }
    fn save(&self) -> String {
        String::new() + "BOp " 
        + self.operator.to_string().as_str() + " "
        + self.lhs.save().as_str()
        + self.rhs.save().as_str()
    }
}

struct TriOp {
    condition: Box<dyn Expression>,
    on_true: Box<dyn Expression>,
    on_false: Box<dyn Expression>,
}

impl Expression for TriOp {
    fn compute(&self) -> isize {
        if self.condition.compute() != 0 { self.on_true.compute() } else { self.on_false.compute() }
    }
    fn pretty_print(&self) -> String {
        String::new()
        + "("
        + self.condition.pretty_print().as_str()
        + "?"
        + self.on_true.pretty_print().as_str()
        + ":"
        + self.on_false.pretty_print().as_str()
        + ")"
    }
    fn save(&self) -> String {
        String::new() + "TOp "
        + self.condition.save().as_str()
        + self.on_true.save().as_str()
        + self.on_false.save().as_str()        
    }
}

fn load_from_terms<'a, T>(terms: &mut T) -> Box<dyn Expression>
    where T: Iterator<Item=&'a str>
{
    match terms.next() {
        Some("C") => {
            let value = str::parse(terms.next().unwrap()).unwrap();
            Box::new(Const{value: value})
        },
        Some("BOp") => {
            let operator = terms.next().unwrap().chars().next().unwrap();
            let lhs = load_from_terms(terms);
            let rhs = load_from_terms(terms);
            Box::new(BinOp{operator: operator, lhs: lhs, rhs: rhs})
        },
        Some("TOp") => {
            let condition = load_from_terms(terms);
            let on_true = load_from_terms(terms);
            let on_false = load_from_terms(terms);
            Box::new(TriOp{condition: condition, on_true: on_true, on_false: on_false})
        },
        _ => panic!(),
    }
}

fn load(input: &str) -> Box<dyn Expression> {
    let mut terms = input.split(" ");
    load_from_terms(&mut terms)
}

fn main() {
    let e = BinOp{operator: '+', lhs: Box::new(Const{value: 2}), rhs: Box::new(Const{value: 2})};

    println!("{}", e.compute());
    println!("{}", e.pretty_print());
    println!("{}", e.save());

    let e2 = load("TOp BOp * BOp + C 2 C 2 C 3 C 1 C 2");
    println!("{}", e2.compute());
    println!("{}", e2.pretty_print());
}
