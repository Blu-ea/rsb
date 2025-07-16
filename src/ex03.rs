use crate::ex03::OperatorRPN::NONE;

#[derive(Copy, Clone)]
enum OperatorRPN{
    FALSE,  // represented by, `0` > no child
    TRUE,   // represented by, `1` > no child

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
    fn add_node(& mut self, value: OperatorRPN) -> bool {
        match self.operator {
            OperatorRPN::TRUE| OperatorRPN::FALSE => false,

            OperatorRPN::NOT => {
                if self.left.is_none(){
                    self.left = Some(Box::new(NodeRPN {operator: NONE, left: None, right: None}));
                }
                self.left.as_deref_mut().unwrap().add_node(value)
            }

            OperatorRPN::AND | OperatorRPN::OR | OperatorRPN::XOR
            | OperatorRPN::IMPLY | OperatorRPN::EQUAL => {
                let left_insert;
                let mut right_insert = false;
                
                if self.left.is_none(){
                    self.left = Some(Box::new(NodeRPN {operator: NONE, left: None, right: None}));
                }
                left_insert = self.left.as_deref_mut().unwrap().add_node(value);
                
                if left_insert == false{
                    if self.right.is_none() {
                        self.right = Some(Box::new(NodeRPN { operator: NONE, left: None, right: None }));
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
            OperatorRPN::FALSE| OperatorRPN::TRUE => true,
            
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

    fn compute(&self) -> bool {
        match self.operator {
            OperatorRPN::FALSE=> false,
            OperatorRPN::TRUE => true,

            OperatorRPN::NOT => !self.left.as_deref().unwrap().compute(),

            OperatorRPN::AND => self.left.as_deref().unwrap().compute() & self.right.as_deref().unwrap().compute(),
            OperatorRPN::OR => self.left.as_deref().unwrap().compute() | self.right.as_deref().unwrap().compute(),
            OperatorRPN::XOR => self.left.as_deref().unwrap().compute() ^ self.right.as_deref().unwrap().compute(),
            OperatorRPN::IMPLY => self.left.as_deref().unwrap().compute() | !self.right.as_deref().unwrap().compute(),
            OperatorRPN::EQUAL => self.left.as_deref().unwrap().compute() == self.right.as_deref().unwrap().compute(),

            OperatorRPN::NONE => unreachable!()
        }
    }
}

pub fn  eval_formula(formula: &str) -> Result<bool, String>{
    let mut root = NodeRPN { operator: NONE, left: None, right: None};

    for val in formula.chars(){
        if "10!&|^>=".find(val) == None {
            return Err(format!("RNP: unauthorised char - {}", val));
        }
    }

    for val in formula.chars().rev() {
        let is_inserted = root.add_node(get_operator(val));
        if is_inserted == false{
            return Err(format!("RNP: cannot compute - {} - missing operator", formula));
        }
    }
    if !root.is_full() {
        return Err(format!("RNP: cannot compute - {} - missing numbers", formula));
    }
    
    Ok(root.compute())
}

fn get_operator(val: char) -> OperatorRPN {
    match val{
        '0' => OperatorRPN::FALSE,
        '1' => OperatorRPN::TRUE,
        '!' => OperatorRPN::NOT,
        '&' => OperatorRPN::AND,
        '|' => OperatorRPN::OR,
        '^' => OperatorRPN::XOR,
        '>' => OperatorRPN::IMPLY,
        '=' => OperatorRPN::EQUAL,
        _ => unreachable!()
    }
}