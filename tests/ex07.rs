// use rsb::ex04::print_truth_table;
use rsb::ex07::sat;

fn test_sat(formula: &str){
    print!("{} -> ", formula);
    let res =  sat(formula);
    if res {
        println!("\x1b[92m{}\x1b[0m", res);
    } else {
        println!("\x1b[91m{}\x1b[0m", res);
    }
    println!();
}

#[test]
fn test_ex07_sat(){
    test_sat("A");
    test_sat("A!");

    test_sat("AA&");
    test_sat("AA!&");
    test_sat("AB&");
}

#[test]
fn test_ex07_bigger_formula(){
    test_sat("ABCDEF>>>&>");
    test_sat("ABCDEF>>>&&");
    test_sat("ABC||ABCABC&&&&&>");
    test_sat("ABCDEFGH&&&&&&&ABCDEFGH&&&&&&&^");
}

#[test]
fn test_ex07_litteral_values() {
    test_sat("1");
    test_sat("0");

    test_sat("ABC10>=>=!");
    test_sat("00000>=>=");

}
