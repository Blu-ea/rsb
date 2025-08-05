use crate::node_rsb::{get_all_var_from_formula, NodeRPN};

pub fn sat(formula: &str) -> bool {
    let formula = formula.to_uppercase();
    let root = NodeRPN::new_tree_from_formula(formula.as_str());
    if root.is_err(){
        panic!("{}", "Error");
    }
    let root = root.unwrap();


    let mut array_var  = get_all_var_from_formula(formula.as_str());
    array_var.sort();
    let max_value: u32 = 0x3FFFFFF >> (26 - array_var.len()); // 2^26 -1

    for i in 0 ..=max_value{
        if root.compute(i, &array_var){
            return true;
        };
    }
    false
}