use crate::node_rsb::{get_all_var_from_formula, NodeRPN};

pub fn  print_truth_table(formula: &str) {
    let formula = formula.to_uppercase();

    let mut array_var  = get_all_var_from_formula(formula.as_str());
    array_var.sort();

    let root = NodeRPN::new_tree_from_formula(formula.as_str());
    match root {
        Ok(root) => root.render_table(&array_var),
        Err(reason) => eprintln!("{}", reason),
    }
}
