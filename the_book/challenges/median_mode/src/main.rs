use std::collections::HashMap;

fn median(n_lst: &mut Vec<i32>) -> f64 {
    n_lst.sort();
    let n_lst_len = n_lst.len();
    let median: f64;
    if n_lst_len % 2 != 0 {
        median = n_lst[n_lst_len / 2] as f64;
    } else {
        median = (n_lst[n_lst_len / 2] as f64 + n_lst[(n_lst_len / 2) - 1] as f64) / 2.0;
    }
    median
}

fn mode(n_lst: &Vec<i32>) -> Vec<i32> {
    let mut highest = 0;
    let mut map = HashMap::new();
    for n_ref in n_lst {
        let count = map.entry(n_ref).or_insert(0);
        *count += 1;
        if *count > highest {
            highest = *count;
        }
    }
    let mut mode: Vec<i32> = Vec::new();
    for (key, value) in &map {
        if *value == highest {
            mode.push(**key)
        }
    }
    mode
}

fn main() {
    let mut n_lst = vec![23, 41, 3, 3, 9, 1092, 59, 1, 9, 10];
    println!("For the vector {:?}", n_lst);
    let median = median(&mut n_lst);
    println!("The median is: {median}");
    let mode = mode(&n_lst);
    println!("The mode is: {:?}", mode);
}
