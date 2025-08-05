use crate::node_rsb::NodeRPN;

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32>{
    let rpn = NodeRPN::new_tree_from_formula(formula);

    if rpn.is_err() || formula.contains('1') || formula.contains('0') {
        panic!("Invalid formula given.")
    }
    let rpn = rpn.unwrap();

    let array_var = get_all_var_from_formula(formula);
    if array_var.len() != sets.len() {
        panic!("The number of set and variables in the formula are not valid.")
    }

    let mut universe : Vec<i32> = Vec::new();
    for sub_set in &sets{
        for val in sub_set{
            if !universe.contains(&val){
                universe.push(*val);
            }
        }
    }
    universe.sort();
    rpn.compute_sets(&universe, &array_var, &sets)
}

fn get_all_var_from_formula(formula: &str) -> Vec<char> {
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