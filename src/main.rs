use std::collections::HashMap;
fn main() {
    let array = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    let array2 = vec![45,105,162,210,280];
    let mut med_array = vec![6, 8, 10, 10, 11, 12, 12, 15, 16, 16, 17, 18, 18, 20, 21, 21, 23, 24, 24, 24, 29, 30, 31, 32, 32, 32, 35, 36, 39, 40];
    let mode_array = vec![21, 19, 62, 21, 66, 28, 66, 48, 79, 59, 28, 62, 63, 63, 48, 66, 59, 66, 94, 79, 19, 94];
    println!("{}", mean(&array));
    println!("{}", mean(&array2));
    println!("{}",median(&mut med_array));
    println!("{:?}",mode(&mode_array));
}

fn mean(int_list: &Vec<i32>) -> i32{
    let mut mean: i32 = 0;
    let lenght = int_list.len() as i32;
    for i in int_list {
        mean += *i;
    }
    mean/lenght
}

fn median(int_list: &mut Vec<i32>) -> i32 {
    int_list.sort();
    let mid = int_list.len() /2;
    int_list[mid]
}

fn mode(int_list: &Vec<i32>) -> HashMap<i32,i32> {

    let mut mode = HashMap::new();
    let mut result = HashMap::new();
    let mut current_key = 0;
    for number in int_list {
        let count = mode.entry(number).or_insert(0);
        *count +=1;
    }

    for (key,value) in mode {
        if result.is_empty() {
            result.insert(*key, value);
            current_key = *key;
        } 
        else if result.get(&current_key) < Some(&value) {
            result.clear();
            result.insert(*key, value);
            current_key = *key;
        }
    }

    result
}

