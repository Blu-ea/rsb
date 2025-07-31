use rsb::ex09::eval_set;

#[test]
fn test_ex09(){

    let set = eval_set("AGZ&|", vec!(
        vec!(0, 1, 2),
        vec!(0, 1, 5),
        vec!(9),
    ));

    println!("{:?}", set);

}