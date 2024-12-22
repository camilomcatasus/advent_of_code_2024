use std::{collections::HashMap, io::Read};


pub fn solve() -> anyhow::Result<()> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut stdin = std::io::stdin();
    stdin.read_to_end(&mut buffer)?;

    let list = String::from_utf8(buffer)?;
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
