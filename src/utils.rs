use std::io::Read;

pub fn get_input() -> anyhow::Result<String> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut stdin = std::io::stdin();
    stdin.read_to_end(&mut buffer)?;

    return Ok(String::from_utf8(buffer)?)
}

pub fn print_bool_map(map: &Vec<Vec<bool>>, false_char: char, true_char: char) {
    map.iter().for_each(|line| {
        let print_line: String = line.iter().map(|b| match b {
            true => true_char,
            false => false_char
        }).collect();

        println!("{print_line}")
    })
}
