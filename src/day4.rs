use crate::utils::get_input;

pub fn solve() -> anyhow::Result<()> {
    let input = get_input()?;
    let char_array : Vec<Vec<char>> =  input.split("\n").map(|line| line.chars().collect()).filter(|arr: &Vec<char>| arr.len() != 0).collect();

    let mut xmas_count = 0;
    let mut x_mas_count = 0;
    for x in 0..char_array[0].len() {
        for y in 0..char_array.len() {

            if let Ok(true) = has_xmas(1, 1, &char_array, x, y) {
                xmas_count += 1;
            }
            if let Ok(true) = has_xmas(1, -1, &char_array, x, y) {
                xmas_count += 1;
            }
            if let Ok(true) = has_xmas(-1, -1, &char_array, x, y) {
                xmas_count += 1;
            }
            if let Ok(true) = has_xmas(-1, 1, &char_array, x, y) {
                xmas_count += 1;
            }
            if let Ok(true) = has_xmas(0, 1, &char_array, x, y) {
                xmas_count += 1;
            }
            if let Ok(true) = has_xmas(0, -1, &char_array, x, y) {
                xmas_count += 1;
            }
            if let Ok(true) = has_xmas(1, 0, &char_array, x, y) {
                xmas_count += 1;
            }
            if let Ok(true) = has_xmas(-1, 0, &char_array, x, y) {
                xmas_count += 1;
            }

            if let Ok(true) = has_x_mas(&char_array, x, y) {
                x_mas_count += 1;
            }
        }
    }
    println!("XMAS COUNT: {xmas_count}");
    println!("X-MAS COUNT: {x_mas_count}");

    Ok(())
}

fn has_xmas(y_dir: isize, x_dir: isize, char_array: &Vec<Vec<char>>, x: usize, y:usize ) -> anyhow::Result<bool> {
    let width = char_array[0].len();
    let height = char_array.len();

    if char_array[y][x] != 'X' {
        return Ok(false);
    }

    let iy = isize::try_from(y)?;
    let ix = isize::try_from(x)?;
    let iwidth = isize::try_from(width)?;
    let iheight = isize::try_from(height)?;

    // Bounds Check
    if iy  + y_dir * 3< 0isize || ix + x_dir * 3< 0isize || ix + x_dir * 3> iwidth - 1|| iy + y_dir * 3> iheight - 1 {
        return Ok(false);
    }

    const MAS: [char;3]= ['M', 'A', 'S'];
    for index_step in 1..4 {
        let y_index = usize::try_from(iy + y_dir * index_step)?;
        let x_index = usize::try_from(ix + x_dir * index_step)?;
        if char_array[y_index][x_index] != MAS[usize::try_from(index_step - 1)?] {
            return Ok(false);
        }
    }
    
    return Ok(true);
}

fn has_x_mas(char_array: &Vec<Vec<char>>, x: usize, y:usize ) -> anyhow::Result<bool> {
    let width = char_array[0].len();
    let height = char_array.len();


    if x + 2> width - 1|| y + 2> height - 1 {
        return Ok(false);
    }

    const MAS1: [[char; 3];3]= [
        ['M', ' ', 'M'],
        [' ', 'A', ' '],
        ['S', ' ', 'S']
    ];
    const MAS2: [[char; 3];3]= [
        ['M', ' ', 'S'],
        [' ', 'A', ' '],
        ['M', ' ', 'S']
    ];
    const MAS3: [[char; 3];3]= [
        ['S', ' ', 'M'],
        [' ', 'A', ' '],
        ['S', ' ', 'M']
    ];
    const MAS4: [[char; 3];3]= [
        ['S', ' ', 'S'],
        [' ', 'A', ' '],
        ['M', ' ', 'M']
    ];

    return Ok(
        compare_matrix(MAS1, char_array, x, y) ||
        compare_matrix(MAS2, char_array, x, y) ||
        compare_matrix(MAS3, char_array, x, y) ||
        compare_matrix(MAS4, char_array, x, y)
    );
}

fn compare_matrix(matrix: [[char; 3]; 3], char_array: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    for y_offset in 0..3 {
        for x_offset in 0..3 {
            if matrix[y_offset][x_offset] != ' ' && matrix[y_offset][x_offset] != char_array[y + y_offset][x + x_offset] {
                return false;
            }
        }
    }
    return true;
}
