use std::collections::HashMap;

const FIB_ZERO: u64 = 1;
const FIB_ONE: u64 = 1;

pub fn basic(n: u64) -> u64 {
    if n == 0 {
        FIB_ZERO
    } else if n == 1 {
        FIB_ONE
    } else {
        basic(n - 1) + basic(n - 2)
    }
}

pub fn dynamic(n: u64) -> u64 {
    let mut map = HashMap::new();
    exec_dyn(n, &mut map)
}

fn exec_dyn(n: u64, map: &mut HashMap<u64, u64>) -> u64 {
    match n {
        0 | 1 => 1,
        n => {
            if map.contains_key(&n) {
                *map.get(&n).unwrap()
            } else {
                let val = exec_dyn(n - 1, map) + exec_dyn(n - 2, map);
                map.insert(n, val);
                val
            }
        }
    }
}
