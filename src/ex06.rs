use crate::node_rsb::NodeRPN;

pub fn conjunctive_normal_form(formula: &str) -> String{
    let root = NodeRPN::new_tree_from_formula(formula);
    if root.is_err(){
        return String::from("ERROR");
    }
    let root = root.unwrap();
    let str = root.to_nnf(false).to_cnf().to_string();
    let mut ret:String = String::new();
    let mut nb_conjecture : u32 = 0;
    for op in str.chars(){
        if op == '&'{
            nb_conjecture += 1;
        } else {
            ret.push(op);
        }
    }
    for _ in 0 .. nb_conjecture {
        ret.push('&');
    }
    ret
}