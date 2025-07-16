use rsb::ex04::print_truth_table;

fn test_truth_table(formula: &str) {
    println!("== -- == Table for formula `\x1b[36m {}\x1b[0m` == -- ==", formula);
    print_truth_table(formula);
    println!()
}

#[test]
fn test_ex04() {
    test_truth_table("AB&AB|&"); // (A & B) & (A | B)
    test_truth_table("AB&AB|&!"); // (A & B) & (A | B)
    test_truth_table("AB&C|"); // (A&B) | C
    test_truth_table("AB^"); // (A ^ B)
    test_truth_table("AB|AB&!&"); // (A | B) & !(A & B) -> Should be an XOR `^` gate 
    test_truth_table("AB>CD|&!EFG>^="); //|  !((A > B)& (C | D))  == (E ^ (F > G))
}

#[test]
fn test_ex04_basic_table() {
    test_truth_table("a");   // A 
    test_truth_table("a!");  // Not A 
    test_truth_table("ab&"); // A & B
    test_truth_table("ab|"); // A | B
    test_truth_table("ab^"); // A ^ B
    test_truth_table("ab>"); // A > B
    test_truth_table("ab="); // A = B
}