use rsb::ex07::sat;

#[test]
fn test_ex07_sat(){
    println!("{}", sat("AB&"));
    println!("{}", sat("AA!&"));
}
