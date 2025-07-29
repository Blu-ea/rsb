use rsb::ex04::print_truth_table;
use rsb::ex06::conjunctive_normal_form;

fn test_ex06_to_cnf(formula: &str) {
    println!();
    println!("== -- == `\x1b[36m{}\x1b[0m` == -- ==", formula);
    print_truth_table(formula);
    let cnf_formual = conjunctive_normal_form(formula);
    println!("\n== -- == `\x1b[33m{}\x1b[0m` == -- ==", cnf_formual);
    print_truth_table(cnf_formual.as_str());
    println!();
    println!();
}


#[test]
fn test_ex06_basic(){
    test_ex06_to_cnf("AB&!");
    test_ex06_to_cnf("AB&");
    test_ex06_to_cnf("AB|!");
    test_ex06_to_cnf("AB|");
    test_ex06_to_cnf("A!!");
}

#[test]
fn test_ex06_basic2(){
    test_ex06_to_cnf("A!B&!");
    test_ex06_to_cnf("A!B&");
    test_ex06_to_cnf("A!B|!");
    test_ex06_to_cnf("A!B|");
    test_ex06_to_cnf("A!!!");
}

#[test]
fn test_ex06_medium(){
    test_ex06_to_cnf("AB&B|!");
    test_ex06_to_cnf("AB&B&!");
    test_ex06_to_cnf("AB|B|!");
    test_ex06_to_cnf("AB|B&!");
}

#[test]
fn test_ex06_other_operatot() {
    test_ex06_to_cnf("AB>");
    test_ex06_to_cnf("AB>!");

    test_ex06_to_cnf("AB^");
    test_ex06_to_cnf("AB^!");

    test_ex06_to_cnf("AB=");
    test_ex06_to_cnf("AB=!");
}

#[test]
fn test_ex06_complex() {
    test_ex06_to_cnf("ABCABC=|>&^");
    test_ex06_to_cnf("ABCABC=|>&^!");

    test_ex06_to_cnf("ABBCAA>=>=>!");
}
