use crate::node_rsb::NodeRPN;

pub fn negation_normal_form(formula: &str) -> String{
    let root = NodeRPN::new_tree_from_formula(formula);
    if root.is_err(){
        return String::from("ERROR");
    } 
    let root = root.unwrap();

    let nnf_root = root.to_nnf(false);
    nnf_root.to_string()
}
