
#[derive(Copy, Clone)]
enum OperatorRPN{
    VAL(char), // represent A .. Z  > 0 child

    NOT,    // represented by, `!` -> 1 child

    AND,    // represented by, `&` => 2 childs
    OR,     // represented by, `|` => 2 childs
    XOR,    // represented by, `^` => 2 childs
    IMPLY,  // represented by, `>` => 2 childs
    EQUAL,  // represented by, `=` => 2 childs

    NONE,   // represent NOTHING | to say that the cell is free to use
}

struct NodeRPN {
    operator: OperatorRPN,

    left: Option<Box<NodeRPN>>,
    right: Option<Box<NodeRPN>>,
}

impl NodeRPN {

}

impl NodeRPN {
    fn add_node(& mut self, value: OperatorRPN) -> bool {
        match self.operator {
            OperatorRPN::VAL(_) => false,

            OperatorRPN::NOT => {
                if self.left.is_none(){
                    self.left = Some(Box::new(NodeRPN {operator: OperatorRPN::NONE, left: None, right: None}));
                }
                self.left.as_deref_mut().unwrap().add_node(value)
            }

            OperatorRPN::AND | OperatorRPN::OR | OperatorRPN::XOR
            | OperatorRPN::IMPLY | OperatorRPN::EQUAL => {
                let left_insert;
                let mut right_insert = false;

                if self.left.is_none(){
                    self.left = Some(Box::new(NodeRPN {operator: OperatorRPN::NONE, left: None, right: None}));
                }
                left_insert = self.left.as_deref_mut().unwrap().add_node(value);

                if left_insert == false{
                    if self.right.is_none() {
                        self.right = Some(Box::new(NodeRPN { operator: OperatorRPN::NONE, left: None, right: None }));
                    }
                    right_insert = self.right.as_deref_mut().unwrap().add_node(value);
                }

                left_insert || right_insert
            }

            OperatorRPN::NONE => {self.operator = value; true}
        }
    }

    fn is_full(&self)->bool{
        match self.operator {
            OperatorRPN::VAL(_) => true,

            OperatorRPN::NOT => self.left.as_deref().unwrap().is_full(),

            OperatorRPN::AND | OperatorRPN::OR | OperatorRPN::XOR
            | OperatorRPN::IMPLY | OperatorRPN::EQUAL => {
                if self.left.is_none() || self.right.is_none() {
                    false
                } else {
                    self.left.as_deref().unwrap().is_full() && self.right.as_deref().unwrap().is_full()
                }
            }

            OperatorRPN::NONE => false
        }
    }

    fn render_table(&self, array_var: Vec<char>) {

        let max_value: u32 = 0x3FFFFFF >> (26 - array_var.len()); // 2^26 -1
        for var in &array_var {
            print!("| {} ", var);
        }
        println!("|| Result |");
        for _ in &array_var {
            print!("|---");
        }
        println!("||--------|");

        for i in 0..=max_value {
            let mut b : u32 = 0;
            while b < array_var.len() as u32 {
                let arg = (i >> b) & 1;
                if arg == 1{
                    print!("| \x1b[92m{}\x1b[0m ",arg);
                } else { 
                    print!("| \x1b[91m{}\x1b[0m ",arg);
                }
                b += 1;
            }
            let res = self.compute(&i, &array_var);
            if res {
                println!("|| \x1b[92m{:>6}\x1b[0m |", res);
            } else {
                println!("|| \x1b[91m{:>6}\x1b[0m |", res);
            }
        }
    }

    fn compute(&self, i: &u32, array_var : &Vec<char>) -> bool {
        match self.operator {
            OperatorRPN::VAL(var) => {
                let position = array_var.iter().position(|&variable| variable == var);
                (i >> position.unwrap()) & 1 == 1 // if 1 -> 1 == 1 so true | if 0 -> false
            },

            OperatorRPN::NOT => !self.left.as_deref().unwrap().compute(i, array_var),

            OperatorRPN::AND => self.left.as_deref().unwrap().compute(i, array_var) & self.right.as_deref().unwrap().compute(i, array_var),
            OperatorRPN::OR => self.left.as_deref().unwrap().compute(i, array_var) | self.right.as_deref().unwrap().compute(i, array_var),
            OperatorRPN::XOR => self.left.as_deref().unwrap().compute(i, array_var) ^ self.right.as_deref().unwrap().compute(i, array_var),
            OperatorRPN::IMPLY => self.left.as_deref().unwrap().compute(i, array_var) | !self.right.as_deref().unwrap().compute(i, array_var),
            OperatorRPN::EQUAL => self.left.as_deref().unwrap().compute(i, array_var) == self.right.as_deref().unwrap().compute(i, array_var),

            OperatorRPN::NONE => unreachable!()
        }
    }
}

pub fn  print_truth_table(formula: &str) {
    let formula = formula.to_uppercase();
    let mut root = NodeRPN { operator: OperatorRPN::NONE, left: None, right: None};

    let mut array_var  = Vec::with_capacity(26);
    for val in formula.chars() {
        if "!&|^>=".find(val) == None {
            if !val.is_ascii_alphabetic() {
                return eprintln!("RNP: unauthorised char - {}", val);
            }
            if !array_var.contains(&val) {
                array_var.push(val);
            }
        }
    }
    array_var.sort();

    for val in formula.chars().rev() {
        let is_inserted = root.add_node(get_operator(val), );
        if is_inserted == false {
            return eprintln!("RNP: cannot compute - {} - missing operator", formula);
        }
    }
    if !root.is_full() {
        return eprintln!("RNP: cannot compute - {} - missing numbers", formula);
    }
    root.render_table(array_var);

}

fn get_operator(val: char) -> OperatorRPN {
    match val{
        'A'..='Z' => OperatorRPN::VAL(val),
        '!' => OperatorRPN::NOT,
        '&' => OperatorRPN::AND,
        '|' => OperatorRPN::OR,
        '^' => OperatorRPN::XOR,
        '>' => OperatorRPN::IMPLY,
        '=' => OperatorRPN::EQUAL,
        _ => unreachable!(),
    }
}