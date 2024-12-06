use std::collections::HashMap;
use std::env;

mod utils;
mod day3;
mod day4;
fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let day: usize = args[1].parse().expect("Could not parse argument");

    match day {
        1 => day_one(),
        2 => day_two(),
        3 => day3::solve(),
        4 => day4::solve(),
        _ => todo!("Unsupported")
    }
}
use std::io::{self, stdin, BufRead, Read};
fn day_one() -> anyhow::Result<()> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut stdin = std::io::stdin();
    stdin.read_to_end(&mut buffer).expect("Could not read input");

    let list = String::from_utf8(buffer).expect("Could not convert input into string");
    let mut distance_count = 0;

    let mut first_list: Vec<isize> = Vec::new();
    let mut second_list: Vec<isize> = Vec::new();
    list.split("\n").for_each(|list_entry| {
        println!("{}", list_entry);
        let split_entry: Vec<&str> = list_entry.split_whitespace().collect();

        if split_entry.len() == 2 {
        let first_num = isize::from_str_radix(split_entry[0], 10).unwrap();
        let second_num = isize::from_str_radix(split_entry[1], 10).unwrap();

            first_list.push(first_num);
            second_list.push(second_num);

        }


    });

    first_list.sort();
    second_list.sort();

    for index in 0..first_list.len() {
        distance_count += (first_list[index] - second_list[index]).abs();

    }

    let mut right_num_count: HashMap<isize, isize> = HashMap::new();

    for num in second_list.iter() {
        match right_num_count.get_mut(num) {
            Some(old) => *old = *old + 1,
            None => {
            right_num_count.insert(*num, 1);
            }
        }

    }

    let mut similarity_score = 0;
    for num in first_list {
        if let Some(matching_count) = right_num_count.get(&num) {
            similarity_score += num * *matching_count;

        }

    }

    println!("Full distance: {distance_count}");
    println!("Similarity Score: {similarity_score}");
    Ok(())
}

fn day_two() -> anyhow::Result<()>{
    fn test_line(nums: &Vec<isize>) -> Option<isize> {
        let mut trend = 0isize;
        for index in 0..(nums.len() - 1) {
            //println!("Index: {index}, is_safe: {is_safe}, trend: {trend}");
            let abs_diff = nums[index].abs_diff(nums[index + 1]);
            let true_diff = nums[index] - nums[index + 1];
            if abs_diff == 1 || abs_diff == 2 || abs_diff == 3 {
                let is_safe = match trend {
                    ..0 => true_diff < 0,
                    0 => { 
                        trend = true_diff;
                        true
                    },
                    1.. =>  true_diff > 0
                };

                if !is_safe {
                    return Some(index as isize);
                }
            }
            else {
               return Some(index as isize);
            }
        }
        return None;
    }
    let line_buf = stdin().lines();
    let mut base_safe_count = 0;
    let mut adjusted_safe_count = 0;
    for line in line_buf {
        let nums : Vec<isize> = line?.split_whitespace().map(|num_str| isize::from_str_radix(num_str, 10).unwrap()).collect();

        if let Some(index) = test_line(&nums) {
            let (left_slice, right_slice)= &nums.split_at(usize::try_from(index)?);
            let mut valid = test_line(&[left_slice, &right_slice[1..]].concat()).is_none();


            if index > 0 && !valid {
                let (left_slice, right_slice)= &nums.split_at(usize::try_from(index - 1)?);
                valid = test_line(&[left_slice, &right_slice[1..]].concat()).is_none();
            }
            if index > 1 && !valid {
                let (left_slice, right_slice)= &nums.split_at(usize::try_from(index - 2)?);
                valid = test_line(&[left_slice, &right_slice[1..]].concat()).is_none();
            }
            if usize::try_from(index)? < nums.len() - 1 && !valid {
                let (left_slice, right_slice)= &nums.split_at(usize::try_from(index + 1)?);
                valid = test_line(&[left_slice, &right_slice[1..]].concat()).is_none();
            }

            if valid {
                adjusted_safe_count+= 1;
            }
        }
        else {
            adjusted_safe_count += 1;
            base_safe_count += 1;
        }

    }

    println!("Safe count: {base_safe_count}");
    println!("Adjusted Safe count: {adjusted_safe_count}");

    Ok(())
}
