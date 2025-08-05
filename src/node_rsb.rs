use std::fmt::{Display, Write};

#[derive(Copy, Clone)]
pub enum OperatorRPN{
    FALSE,
    TRUE,
    VAL(char), // represent A .. Z  > 0 child

    NOT,    // represented by, `!` -> 1 child

    AND,    // represented by, `&` => 2 childs
    OR,     // represented by, `|` => 2 childs
    XOR,    // represented by, `^` => 2 childs
    IMPLY,  // represented by, `>` => 2 childs
    EQUAL,  // represented by, `=` => 2 childs

    NONE,   // represent NOTHING | to say that the cell is free to use
}

impl OperatorRPN {

    fn new_from_char(val: char) -> OperatorRPN {
        match val{
            '0' => OperatorRPN::FALSE,
            '1' => OperatorRPN::TRUE,
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
}

#[derive(Clone)]
pub struct NodeRPN {
    operator: OperatorRPN,

    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
}

impl Default for NodeRPN {
    fn default() -> Self {
        NodeRPN{
            operator: OperatorRPN::NONE,
            left: None,
            right: None
        }
    }
}

impl Display for NodeRPN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.operator {
            OperatorRPN::VAL(_) | OperatorRPN::TRUE | OperatorRPN::FALSE
            => f.write_char(self.to_char()),

            OperatorRPN::NOT
            => write!(f, "{}{}",
                      self.left.as_deref().unwrap_or(&NodeRPN { operator: OperatorRPN::VAL('*'), left: None, right: None }),
                      self.to_char()
            ),

            OperatorRPN::AND | OperatorRPN::OR | OperatorRPN::XOR | OperatorRPN::IMPLY | OperatorRPN::EQUAL
            => write!(f, "{}{}{}",
                      self.right.as_deref().unwrap_or(&NodeRPN { operator: OperatorRPN::VAL('*'), left: None, right: None }),
                      self.left.as_deref().unwrap_or(&NodeRPN { operator: OperatorRPN::VAL('*'), left: None, right: None }),
                      self.to_char()
            ),

            OperatorRPN::NONE => unimplemented!(),
        }
    }
}

impl NodeRPN {
    pub fn new_tree_from_formula(formula: &str) -> Result<NodeRPN, String> { // todo: check for not authorised char in the formula
        let mut root = NodeRPN::default();


        for val in formula.chars() {
            if "10!&|^>=".find(val) == None && !val.is_alphabetic(){
                return Err(format!("RNP: unauthorised char - {}", val));
            }
        }
            for val in formula.chars().rev() {
            let is_inserted = root.add_node(OperatorRPN::new_from_char(val), );
            if is_inserted == false {
                return Err(format!("RNP: cannot compute - {} - missing operator", formula));
            }
        }
        if !root.is_full() {
            return Err(format!("RNP: cannot compute - {} - missing numbers", formula));
        }

        Ok(root)
    }

    fn add_node(&mut self, value: OperatorRPN) -> bool {
        match self.operator {
            OperatorRPN::VAL(_) | OperatorRPN::TRUE | OperatorRPN::FALSE
            => false,

            OperatorRPN::NOT => {
                if self.left.is_none() {
                    self.left = Some(Box::new(NodeRPN::default()));
                }
                self.left.as_deref_mut().unwrap().add_node(value)
            }

            OperatorRPN::AND | OperatorRPN::OR | OperatorRPN::XOR
            | OperatorRPN::IMPLY | OperatorRPN::EQUAL => {
                let left_insert;
                let mut right_insert = false;

                if self.left.is_none() {
                    self.left = Some(Box::new(NodeRPN::default()));
                }
                left_insert = self.left.as_deref_mut().unwrap().add_node(value);

                if left_insert == false {
                    if self.right.is_none() {
                        self.right = Some(Box::new(NodeRPN::default()));
                    }
                    right_insert = self.right.as_deref_mut().unwrap().add_node(value);
                }

                left_insert || right_insert
            }

            OperatorRPN::NONE => {
                self.operator = value;
                true
            }
        }
    }

    fn is_full(&self) -> bool {
        match self.operator {
            OperatorRPN::VAL(_) | OperatorRPN::TRUE | OperatorRPN::FALSE
            => true,

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

    pub fn render_table(&self, array_var: &[char]) {
        let max_value: u32 = 0x3FFFFFF >> (26 - array_var.len()); // 2^26 -1
        for var in array_var {
            print!("| {} ", var);
        }
        println!("|| Result |");
        for _ in array_var {
            print!("|---");
        }
        println!("||--------|");

        for i in 0..=max_value {
            let mut b: u32 = 0;
            while b < array_var.len() as u32 {
                let arg = (i >> b) & 1;
                if arg == 1 {
                    print!("| \x1b[92m{}\x1b[0m ", arg);
                } else {
                    print!("| \x1b[91m{}\x1b[0m ", arg);
                }
                b += 1;
            }
            let res = self.compute(i, &array_var);
            if res {
                println!("|| \x1b[92m{:>6}\x1b[0m |", res);
            } else {
                println!("|| \x1b[91m{:>6}\x1b[0m |", res);
            }
        }
    }

    pub fn compute(&self, i: u32, array_var: &[char]) -> bool {
        match self.operator {
            OperatorRPN::TRUE => true,
            OperatorRPN::FALSE => false,
            OperatorRPN::VAL(var) => {
                let position = array_var.iter().position(|&variable| variable == var);
                ((i >> position.unwrap()) & 1)== 1 // if 1 -> 1 == 1 so true | if 0 -> false
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

    fn to_char(&self) -> char {
        match self.operator {
            OperatorRPN::FALSE => '0',
            OperatorRPN::TRUE => '1',
            OperatorRPN::VAL(val) => val,
            OperatorRPN::NOT => '!',
            OperatorRPN::AND => '&',
            OperatorRPN::OR => '|',
            OperatorRPN::XOR => '^',
            OperatorRPN::IMPLY => '>',
            OperatorRPN::EQUAL => '=',
            OperatorRPN::NONE => '*',
        }
    }

    //noinspection D
    pub fn to_nnf(&self, is_negate: bool) -> NodeRPN {
        match self.operator {
            OperatorRPN::TRUE=> {
                if is_negate {
                    NodeRPN {operator: OperatorRPN::FALSE, left: None, right: None}
                } else {
                    NodeRPN {operator: OperatorRPN::TRUE, left: None, right: None}
                }
            }
            OperatorRPN::FALSE => {
                if !is_negate {
                    NodeRPN {operator: OperatorRPN::FALSE, left: None, right: None}
                } else {
                    NodeRPN {operator: OperatorRPN::TRUE, left: None, right: None}
                }
            }
            OperatorRPN::VAL(_) => {
                let mut nnf_node = NodeRPN { operator: OperatorRPN::NONE, left: None, right: None };
                if is_negate {
                    nnf_node.add_node(OperatorRPN::NOT);
                }
                nnf_node.add_node(self.operator);
                nnf_node
            }

            OperatorRPN::NOT => {
                self.left.as_deref().unwrap().to_nnf(!is_negate)
            }

            OperatorRPN::AND => {
                NodeRPN {
                    operator: if is_negate { OperatorRPN::OR } else { OperatorRPN::AND },
                    left: Some(Box::new(self.left.as_deref().unwrap().to_nnf(is_negate))),
                    right: Some(Box::new(self.right.as_deref().unwrap().to_nnf(is_negate))),
                }
            }
            OperatorRPN::OR => {
                NodeRPN {
                    operator: if is_negate { OperatorRPN::AND } else { OperatorRPN::OR },
                    left: Some(Box::new(self.left.as_deref().unwrap().to_nnf(is_negate))),
                    right: Some(Box::new(self.right.as_deref().unwrap().to_nnf(is_negate))),
                }
            }

            OperatorRPN::XOR => { // -> (!A | !B) & (A | B)
                NodeRPN {
                    operator: if is_negate { OperatorRPN::OR } else { OperatorRPN::AND },
                    left: Some(Box::new(NodeRPN {
                        operator: if is_negate { OperatorRPN::AND } else { OperatorRPN::OR },
                        left: Some(Box::new(self.left.as_deref().unwrap().to_nnf(is_negate))),
                        right: Some(Box::new(self.right.as_deref().unwrap().to_nnf(is_negate))),
                    })),
                    right: Some(Box::new(NodeRPN {
                        operator: if is_negate { OperatorRPN::AND } else { OperatorRPN::OR },
                        left: Some(Box::new(self.left.as_deref().unwrap().to_nnf(!is_negate))),
                        right: Some(Box::new(self.right.as_deref().unwrap().to_nnf(!is_negate))),
                    })),
                }
            }
            OperatorRPN::IMPLY => {
                NodeRPN {
                    operator: if is_negate { OperatorRPN::AND } else { OperatorRPN::OR },
                    left: Some(Box::new(self.left.as_deref().unwrap().to_nnf(is_negate))),
                    right: Some(Box::new(self.right.as_deref().unwrap().to_nnf(!is_negate))),
                }
            }

            OperatorRPN::EQUAL => {
                NodeRPN {
                    operator: if is_negate { OperatorRPN::AND } else { OperatorRPN::OR },
                    left: Some(Box::new(NodeRPN {
                        operator: if is_negate { OperatorRPN::OR } else { OperatorRPN::AND },
                        left: Some(Box::new(self.left.as_deref().unwrap().to_nnf(!is_negate))),
                        right: Some(Box::new(self.right.as_deref().unwrap().to_nnf(!is_negate))),
                    })),
                    right: Some(Box::new(NodeRPN {
                        operator: if is_negate { OperatorRPN::OR } else { OperatorRPN::AND },
                        left: Some(Box::new(self.left.as_deref().unwrap().to_nnf(is_negate))),
                        right: Some(Box::new(self.right.as_deref().unwrap().to_nnf(is_negate))),
                    })),
                }
            }

            OperatorRPN::NONE => unreachable!()
        }
    }
}

impl NodeRPN { // ex06 -> CNF
    pub fn to_cnf(&self) -> NodeRPN {
        match self.operator {
            OperatorRPN::TRUE => NodeRPN{
                operator: OperatorRPN::TRUE,
                left: None,
                right: None,
            },
            OperatorRPN::FALSE => NodeRPN{
                operator: OperatorRPN::FALSE,
                left: None,
                right: None,
            },
            OperatorRPN::VAL(var) => {
                NodeRPN {
                    operator: OperatorRPN::VAL(var),
                    left: None,
                    right: None,
                }
            }
            OperatorRPN::NOT => {
                if !matches!(self.left.as_deref().unwrap().operator, OperatorRPN::VAL(_)) {
                    unreachable!("Current formula should be translated to NNF first");
                }
                NodeRPN {
                    operator: OperatorRPN::NOT,
                    left: Some(Box::new(self.left.as_deref().unwrap().to_cnf())),
                    right: None,
                }
            }
            OperatorRPN::AND => {
                NodeRPN {
                    operator: OperatorRPN::AND,
                    left: Some(Box::new(self.left.as_deref().unwrap().to_cnf())),
                    right: Some(Box::new(self.right.as_deref().unwrap().to_cnf())),
                }
            }
            OperatorRPN::OR => {
                let left = Box::new(self.left.as_deref().unwrap().to_cnf());
                let right = Box::new(self.right.as_deref().unwrap().to_cnf());
                Self::distribute_or(*left, *right)
            }
            _ => unreachable!("Current formula should be translated to NNF first")
        }
    }

    fn distribute_or(a: NodeRPN, b: NodeRPN) -> NodeRPN {
        match (a.operator, b.operator) {
            (OperatorRPN::AND, _) => NodeRPN {
                operator: OperatorRPN::AND,
                left: Some(Box::new(NodeRPN::distribute_or(*a.left.unwrap(), b.clone()))),
                right: Some(Box::new(NodeRPN::distribute_or(*a.right.unwrap(), b))),
            },

            (_, OperatorRPN::AND) => NodeRPN {
                operator: OperatorRPN::AND,
                left: Some(Box::new(NodeRPN::distribute_or(a.clone(), *b.left.unwrap()))),
                right: Some(Box::new(NodeRPN::distribute_or(a, *b.right.unwrap()))),
            },

            _ => NodeRPN {
                operator: OperatorRPN::OR,
                left: Some(Box::new(a)),
                right: Some(Box::new(b)),
            },
        }
    }
}


impl NodeRPN{ // ex09 -> Eval set

    pub fn compute_sets(&self, universe: &Vec<i32>,array_var: &Vec<char>,  sets: &Vec<Vec<i32>>) -> Vec<i32>{
        let left = self.left.as_deref();
        let right = self.right.as_deref();
        match self.operator {
            OperatorRPN::VAL(var) => {
                let position = array_var.iter().position(|&variable| variable == var);
                sets[position.unwrap()].clone()
            }

            OperatorRPN::NOT => {
                NodeRPN::compute_sets_not(left.unwrap().compute_sets(universe, array_var, sets), universe)
            }

            OperatorRPN::AND => {
                NodeRPN::compute_sets_and(
                    left.unwrap().compute_sets(universe, array_var, sets),
                    right.unwrap().compute_sets(universe, array_var, sets),
                )
            }
            OperatorRPN::OR => {
                NodeRPN::compute_sets_or(
                    left.unwrap().compute_sets(universe, array_var, sets),
                    right.unwrap().compute_sets(universe, array_var, sets),
                )
            }
            OperatorRPN::XOR => {
                NodeRPN::compute_sets_xor(
                    left.unwrap().compute_sets(universe, array_var, sets),
                    right.unwrap().compute_sets(universe, array_var, sets),
                )
            }
            OperatorRPN::IMPLY => {
                NodeRPN::compute_sets_or(
                    left.unwrap().compute_sets(universe, array_var, sets),
                    NodeRPN::compute_sets_not(right.unwrap().compute_sets(universe, array_var, sets), universe),
                )
            }
            OperatorRPN::EQUAL => {
                NodeRPN::compute_sets_equal(
                    left.unwrap().compute_sets(universe, array_var, sets),
                    right.unwrap().compute_sets(universe, array_var, sets),
                    universe,
                )
            }

            OperatorRPN::FALSE | OperatorRPN::TRUE => panic!("To evaluate set we cannot have True or False Statement"),
            OperatorRPN::NONE => unreachable!(),
        }
    }

    fn compute_sets_not(set: Vec<i32>, universe: &Vec<i32>) -> Vec<i32>{
        let mut new_set = Vec::new();
        for n in universe{
            if !set.contains(&n){
                new_set.push(n.clone());
            }
        }
        new_set
    }

    fn compute_sets_and(set_l: Vec<i32>, set_r: Vec<i32>,) -> Vec<i32>{
        let mut new_set = Vec::new();
        for n in set_l {
            if set_r.contains(&n){
                new_set.push(n);
            }
        }
        new_set
    }

    fn compute_sets_or(set_l: Vec<i32>, set_r: Vec<i32>) -> Vec<i32>{
        // vec![set_l, set_r].concat()
        let mut new_set = set_l.clone();

        for n in set_r {
            if !set_l.contains(&n){
                new_set.push(n);
            }
        }
        new_set
    }
    fn compute_sets_xor(set_l: Vec<i32>, set_r: Vec<i32>) -> Vec<i32>{
        let mut new_set = Vec::new();
        for n in &set_l {
            if !set_r.contains(&n){
                new_set.push(*n);
            }
        }
        for n in &set_r {
            if !set_l.contains(&n){
                new_set.push(*n);
            }
        }
        new_set
    }
    fn compute_sets_equal(set_l: Vec<i32>, set_r: Vec<i32>, universe: &Vec<i32>) -> Vec<i32>{
        let mut new_set = Vec::new();
        for n in universe{
            if set_l.contains(&n) == set_r.contains(&n){
                new_set.push(*n);
            }
        }
        new_set
    }


}

pub fn get_all_var_from_formula(formula: &str) -> Vec<char> {
    let mut array_var  = Vec::with_capacity(26);
    for val in formula.chars() {
        if "!&|^>=".find(val) == None {
            if !array_var.contains(&val) {
                array_var.push(val);
            }
        }
    }

    array_var
}