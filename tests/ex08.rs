use rsb::ex08::powerset;

fn test_powerset(set: Vec<i32>, mut expected: Vec<Vec<i32>>) {
    println!("For set : {:?}", set);
    println!("expected : {:?}", expected);
    let mut result = powerset(set);
    println!("result   : {:?}",  result);
    assert_eq!(result.sort(), expected.sort());
    println!("\x1b[92mResult match!\x1b[0m")
}
// silly test (cannot allocate the size needed)
// let ps = powerset(vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58));

#[test]
fn test_ex08(){
    test_powerset(
        vec![],
        vec![
            vec![],
        ],
    );
    
    test_powerset(
        vec![1],
        vec![
            vec![],
            vec![1],
        ],
    );

    test_powerset(
        vec![1,2],
        vec![
            vec![],
            vec![1],
            vec![2],
            vec![1,2],
        ],
    );

    test_powerset(
        vec![1,2,3],
        vec![
            vec![],
            vec![1],
            vec![2],
            vec![3],
            vec![1,2],
            vec![1,3],
            vec![2,3],
            vec![1,2,3],
        ]
    );

    test_powerset(
        vec![5,-2,25],
        vec![
            vec![],
            vec![5],
            vec![-2],
            vec![25],
            vec![5,-2],
            vec![5,25],
            vec![-2,25],
            vec![5,-2,25],
        ]
    );

    test_powerset(
        vec![1,2,3,4,5],
        vec![
            vec![], vec![1], vec![2], vec![3], vec![4], vec![5],
            vec![1,2], vec![1,3], vec![1,4], vec![1,5], vec![2,3], vec![2,4], vec![2,5],
            vec![3,4], vec![3,5], vec![4,5],
            vec![1,2,3], vec![1,2,4], vec![1,2,5], vec![1,3,4], vec![1,3,5], vec![1,4,5],
            vec![2,3,4], vec![2,3,5], vec![2,4,5], vec![3,4,5],
            vec![1,2,3,4], vec![1,2,3,5], vec![1,2,4,5], vec![1,3,4,5], vec![2,3,4,5],
            vec![1,2,3,4,5],
        ]
    );
}

fn show_powerset(set: Vec<i32>){
    println!("=================================");
    println!("For set : {:?}", set);
    println!("result  : {:?}",  powerset(set));
}

#[test]
fn test_ex08_show_random_set(){
    show_powerset(vec![1]);
    show_powerset(vec![1,2]);
    show_powerset(vec![1,2,3]);
    show_powerset(vec![1,2,3,4]);
    show_powerset(vec![1,2,3,4,5]);
    show_powerset(vec![1,2,3,4,5,6]);
    show_powerset(vec![1,2,3,4,5,6,7]);
    show_powerset(vec![1,2,3,4,5,6,7,8]);
    show_powerset(vec![1,2,3,4,5,6,7,8,9]);
    show_powerset(vec![1,2,3,4,5,6,7,8,9,10]);
}