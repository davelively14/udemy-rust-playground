fn take_ownership_sum(v: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for value in v {
        sum += value;
    }
    sum
}

fn borrow_sum(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for value in v {
        sum += *value;
    }
    sum
}

fn cap_values_owned(max: i32, mut v: Vec<i32>) -> Vec<i32> {
    for index in 0..v.len() {
        if v[index] > max {
            v[index] = max;
        }
    }
    v
}

fn cap_values_borrowed(max: i32, v: &mut Vec<i32>) {
    for index in 0..v.len() {
        if v[index] > max {
            v[index] = max;
        }
    }
}

pub fn run_examples() {
    let values = vec![1, 2, 3, 4, 5];
    let sum = take_ownership_sum(values);
    println!("Can't tell you length of values. It was moved to another fn");
    println!("Took ownership, values: {}", sum);

    let new_values = vec![1, 2, 3, 4, 5];
    let new_sum = borrow_sum(&new_values);
    println!("Borrowed {} values, sum is {}", new_values.len(), new_sum);

    println!("Owned");

    let mut third_values = vec![1, 2, 3, 10000, 5];
    third_values = cap_values_owned(10, third_values);

    for v in third_values {
        println!("{}", v);
    }

    println!("Borrowed");
    let mut fourth_values = vec![1, 2, 3, 10000, 5];
    cap_values_borrowed(10, &mut fourth_values);

    for v in fourth_values {
        println!("{}", v);
    }
}
