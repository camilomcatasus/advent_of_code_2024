use std::io::stdin ;

pub fn solve() -> anyhow::Result<()>{
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
