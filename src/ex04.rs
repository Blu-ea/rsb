use crate::node_rsb::{NodeRPN};

pub fn  print_truth_table(formula: &str) {
    let formula = formula.to_uppercase();

    let mut array_var  = Vec::with_capacity(26);
    for val in formula.chars() {
        if val.is_alphabetic() && !array_var.contains(&val) {
            array_var.push(val);
        }
    }
    array_var.sort();

    let root = NodeRPN::new_tree_from_formula(formula.as_str());
    match root {
        Ok(root) => root.render_table(&array_var),
        Err(reason) => eprintln!("{}", reason),
    }
}
