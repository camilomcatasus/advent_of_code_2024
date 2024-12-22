use crate::utils::get_input;

pub fn solve() -> anyhow::Result<()> {
    let input = get_input()?;

    let lines : Vec<&str> = input.split("\r\n").collect();

    let split_index = lines.iter().enumerate().find(|(index, &line)| line == "").expect("Empty line separator missing");

    let (rule_lines, group_lines) = lines.split_at(split_index.0);

    println!("{:?}", group_lines);
    // Here we use a table instead HashSet because the input isn't super complicated and fits
    // neatly into the table
    let mut rule_table = [[false; 100]; 100];

    for rule_line in rule_lines.iter() {
        let (left, right) = rule_line.split_once("|").unwrap();
    
        let left_num = usize::from_str_radix(left.trim(), 10)?;
        let right_num = usize::from_str_radix(right.trim(), 10)?;

        rule_table[left_num][right_num] = true;
    }


    let mut running_sum = 0;
    let mut running_sum_2 = 0;
    for group_line in group_lines.iter().skip(1) {
        if group_line == &"" {
            continue;
        }
        let mut group : Vec<usize> = group_line.split(",").map(|num_str| usize::from_str_radix(num_str, 10).unwrap()).collect();

        let mut is_correct = true;
        for i in (0..group.len() - 1).rev() {
            for j in (i+1)..group.len() {
                is_correct &= !rule_table[group[j]][group[i]];
            }
        }

        if is_correct {
            running_sum += group[group.len()/2];
        }
        else {
            group.sort_by(|a, b| match rule_table[*a][*b] {
                true => std::cmp::Ordering::Less,
                false => std::cmp::Ordering::Greater
            });
            running_sum_2 += group[group.len()/2];
        }
    }

    println!("Part1 running sum: {running_sum}");
    println!("Part2 running sum: {running_sum_2}");


    Ok(())
}
