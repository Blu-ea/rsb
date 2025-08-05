use crate::node_rsb::{NodeRPN};

pub fn  eval_formula(formula: &str) -> Result<bool, String>{
    let root = NodeRPN::new_tree_from_formula(formula);

    Ok(root?.compute(0u32, &[]))
}
