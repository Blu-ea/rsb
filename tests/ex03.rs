use rsb::ex03::eval_formula;

fn test_rpn(formula: &str){
    print!("{} -> ", formula);
    let result = eval_formula(formula);
    if result.is_ok(){
        println!("{}", result.unwrap());
    } else {
        eprintln!("{}", result.unwrap_err());
    }
}

fn test_rnp_cmp(formula: &str, expected_result : bool){
    print!("expected : {} -> {:?}", formula, expected_result);

    let result = eval_formula(formula);
    if result.is_ok(){
        assert_eq!(result.unwrap(), expected_result);
        println!(" ---   PASS")
    } else {
        eprintln!("{}", result.unwrap_err());
        assert!(false);
    }
}

fn test_table(operator: char){
    println!("Table for operator {}", operator);
    test_rpn(format!("11{}", operator).as_str());
    test_rpn(format!("10{}", operator).as_str());
    test_rpn(format!("01{}", operator).as_str());
    test_rpn(format!("00{}", operator).as_str());
    println!();
}


#[test]
fn test_ex03_print_table() {
    println!("Table for operator Nothing");
    test_rpn("1");
    test_rpn("0");
    println!();

    println!("Table for operator !");
    test_rpn("1!");
    test_rpn("0!");
    println!();
    
    test_table('&');
    test_table('|');
    test_table('^');
    test_table('>');
    test_table('=');
}

#[test]
fn test_ex03_eval(){
    test_rnp_cmp("1011||=", true);
    test_rnp_cmp("1", true);
    test_rnp_cmp("0", false);
    
    test_rnp_cmp("10|1&",   true);
    test_rnp_cmp("101|&",   true);
    
    test_rnp_cmp("110&&",   false);
    test_rnp_cmp("110&&!",  true);
}

#[test]
fn test_ex03_err(){
    assert!(eval_formula("111111").is_err());       // To much numbers
    assert!(eval_formula("11&1&||||||").is_err());  // To much operator
    assert!(eval_formula("&11").is_err());          // To much operator
    assert!(eval_formula("59|").is_err());          // Wrong input
}