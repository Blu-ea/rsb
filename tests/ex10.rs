use rsb::ex10::map;

fn test_map(x: u16, y: u16){
    println!("For x: {}, y: {} -> {}", x, y, map(x,y));
}

#[test]
fn test_ex10(){
    test_map(0, 0);
    test_map(1, 0);
    test_map(1, 1);
    test_map(0, 1);
    test_map(0, u16::MAX);
    test_map(u16::MAX, u16::MAX);
    test_map(u16::MAX, 0);
}
