use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let input_vec = vec![-3, 4, 3, 90];
    let target = 0;

    let result_vec = two_sum(input_vec.clone(), target);

    println!("result: {:?}", result_vec);
    
    print!("result: {:?}", get_concat(input_vec));
}

fn get_concat(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0; nums.len() * 2];
    let half = nums.len() / 2;
    println!("{}", half);

    for (i, num) in nums.iter().enumerate() {
        result[i] = *num;
        result[half + i] = *num;
    }

    result
    // nums.to_owned().append(&mut nums)
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut track_map: HashMap<i32, i32> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let num_target: i32 = target - num;
        // if &target >= num {
        //     num_target = target - num;
        // } else {
        //     num_target = num - target;
        // }
        println!("target: {}", num_target);

        if track_map.contains_key(&num_target) {
            return vec![*track_map.get(&num_target).unwrap(), i as i32];
        }
        track_map.insert(*num, i.try_into().unwrap());
    }

    vec![]
}
