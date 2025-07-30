use crate::node_rsb::NodeRPN;

pub fn sat(formula: &str) -> bool {
    let formula = formula.to_uppercase();
    let root = NodeRPN::new_tree_from_formula(formula.as_str());
    if root.is_err(){
        panic!("{}", "Error");
    }
    let root = root.unwrap();


    let mut array_var  = Vec::with_capacity(26);
    for val in formula.chars() {
        if val.is_alphabetic() && !array_var.contains(&val) {
            array_var.push(val);
        }
    }
    array_var.sort();
    let max_value: u32 = 0x3FFFFFF >> (26 - array_var.len()); // 2^26 -1

    for i in 0 ..=max_value{
        if root.compute(&i, &array_var){
            return true;
        };
    }
    false
}