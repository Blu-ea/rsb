use rsb::*;

fn computed_result (n : u32,result : u32) {
    let computed_result = ex02::gray_code(n);
    println!("n: {}, computed result: {}", n, computed_result);
    assert_eq!(computed_result, result);
}

#[test]
fn test_ex00() {
    computed_result(0,0);
    computed_result(1,1);
    computed_result(2,3);
    computed_result(3,2);
    computed_result(4,6);
    computed_result(5,7);
    computed_result(6,5);
    computed_result(7,4);
    computed_result(8,12);
}