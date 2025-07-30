pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>>{
    if set.len() > 58{
        panic!("Cannot compute the powerset of current set (would overflow the vector)")
    }

    let powerset_size = 1usize << set.len();
    let mut powerset: Vec<Vec<i32>> = Vec::with_capacity(powerset_size);

    for i in 0..powerset_size {
        let mut new_set: Vec<i32> = Vec::new();

        for b in 0 .. set.len() {
            let arg = (i >> b) & 1;
            if arg == 1 {
                new_set.push(set[b]);
            }
        }
        powerset.push(new_set);
    }
    powerset
}