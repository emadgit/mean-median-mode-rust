use std::collections::HashMap;

fn main() {
    let list = vec![
        55, 13, 33, 6, 185, 39, 11, 23, 55, 33, 67, 2, 77, 12, 55, 7, 44, 51, 67, 33,
    ];
    mean(&list); // 45.04762
    median(list.clone()); // 41
    mode(list.clone()); // Mode for the input are the follwoing: [33, 55] Which repated 3 times
}

fn mean(list: &Vec<i32>) -> f32 {
    let sum: i32 = list.iter().sum();
    let mean: f32 = sum as f32 / list.len() as f32;
    return mean;
}

fn median(mut list: Vec<i32>) -> i32 {
    list.sort();
    let median: i32;
    let mid = list.len() / 2;
    if list.len() % 2 == 0 {
        // We have two middle numbers, so we need to get the mean of them!
        let mut middle_index_left = match list.get(mid) {
            None => None,
            Some(i) => Some(i),
        };
        let mut middle_index_right = match list.get(mid + 1) {
            None => None,
            Some(i) => Some(i),
        };
        let middle_index_vec = vec![
            middle_index_left.get_or_insert(&0).clone(),
            middle_index_right.get_or_insert(&0).clone(),
        ];

        median = mean(&middle_index_vec) as i32;
    } else {
        let mut middle_index = match list.get(mid) {
            None => None,
            Some(i) => Some(i),
        };
        median = middle_index.get_or_insert(&0).clone();
    };

    return median;
}

fn mode(list: Vec<i32>) -> Vec<i32> {
    let mut mode = HashMap::new();
    for item in list.iter() {
        let count = mode.entry(item).or_insert(0);
        *count += 1;
    }
    let mut max_key = Vec::new();
    let mut max = 0;

    for (_key, value) in mode.clone() {
        if value >= max {
            max = value;
        };
    }

    for (key, value) in mode.clone() {
        if value == max {
            max_key.push(key.clone());
        }
    }
    println!(
        "Mode for the input are the follwoing: {:?} Which repated {} times",
        max_key.clone(),
        max
    );

    return max_key;
}
