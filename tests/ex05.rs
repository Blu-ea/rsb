/*
    AB&! -> A!B!|   // Put the ! close to the numbers
    AB|! -> A!B!&   //  ^ same here

    AB>  ->   A!B|        // just a conversion
    AB=  -> AB&A!B!&|   // Just a conversion
-
    How to have the NNF
    -> remove all the unwanted notation (=,>,^)
    -> put the ! sign just next to the variables and if an other ! sign is found, they cancel ou, if a & or | is found, they swap
*/
use rsb::ex04::print_truth_table;
use rsb::ex05::negation_normal_form;

fn test_ex05_to_nnf(formula: &str) {
    println!();
    println!("== -- == `\x1b[36m{}\x1b[0m` == -- ==", formula);
    print_truth_table(formula);
    let nnf_formual = negation_normal_form(formula);
    println!("\n== -- == `\x1b[33m{}\x1b[0m` == -- ==", nnf_formual);
    print_truth_table(nnf_formual.as_str());
    println!();
    println!();
}


#[test]
fn test_ex05_basic(){
    test_ex05_to_nnf("AB&!");
    test_ex05_to_nnf("AB&");
    test_ex05_to_nnf("AB|!");
    test_ex05_to_nnf("AB|");
    test_ex05_to_nnf("A!!");
}

#[test]
fn test_ex05_basic2(){
    test_ex05_to_nnf("A!B&!");
    test_ex05_to_nnf("A!B&");
    test_ex05_to_nnf("A!B|!");
    test_ex05_to_nnf("A!B|");
    test_ex05_to_nnf("A!!!");
}

#[test]
fn test_ex05_medium(){
    test_ex05_to_nnf("AB&B|!");
    test_ex05_to_nnf("AB&B&!");
    test_ex05_to_nnf("AB|B|!");
    test_ex05_to_nnf("AB|B&!");
}

#[test]
fn test_ex05_other_operatot() {
    test_ex05_to_nnf("AB>");
    test_ex05_to_nnf("AB>!");

    test_ex05_to_nnf("AB^");
    test_ex05_to_nnf("AB^!");

    test_ex05_to_nnf("AB=");
    test_ex05_to_nnf("AB=!");
}

#[test]
fn test_ex05_complex() {
    test_ex05_to_nnf("ABCABC=|>&^");
    test_ex05_to_nnf("ABCABC=|>&^!");
    
    test_ex05_to_nnf("ABBCAA>=>=>!");
}
