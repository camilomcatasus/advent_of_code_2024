use std::io::Read;

use anyhow::{bail, Context};

pub fn solve() -> anyhow::Result<()> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut stdin = std::io::stdin();
    stdin.read_to_end(&mut buffer)?;
    let full_text : Vec<char> = String::from_utf8(buffer)?.chars().collect();
    let mut cursor = 0;
    let mut total = 0;
    let mut enabled = true;
    while cursor < full_text.len() {
        match try_parse(&full_text, &mut cursor) {
            Ok(pair) => total = total + pair.0 * pair.1,
            Err(_) => cursor += 1
        }
    }

    let mut adjusted_total = 0;
    let mut cursor = 0;
    while cursor < full_text.len() {
        if cursor < full_text.len() - 4 && match_chars(&full_text[cursor..], "do()") {
            enabled = true;
            cursor += 4;
        }
        else if cursor < full_text.len() - 7 && match_chars(&full_text[cursor..], "don't()") {
            enabled = false;
            cursor += 7;

        }
        else if enabled {
            match try_parse(&full_text, &mut cursor) {
                Ok(pair) => adjusted_total = adjusted_total + pair.0 * pair.1,
                Err(_) => cursor += 1
            }
        }
        else {
            cursor += 1;
        }
    }


    println!("Total: {total}");
    println!("Adjusted Total: {adjusted_total}");
    Ok(())
}

fn match_chars(slice: &[char], value: &str) -> bool {
    let value_chars : Vec<char> = value.chars().collect();
    for index in 0..value.len() {
        if slice[index] != value_chars[index] {
            return false;
        }
    }
    return true;

}

fn try_parse(slice: &[char], cursor: &mut usize) -> anyhow::Result<(isize, isize)> {
    if !(slice[*cursor] == 'm' && slice[*cursor + 1] == 'u' && slice[*cursor + 2] == 'l' && slice [*cursor + 3] == '(') {
        bail!("Does not match");
    }
    *cursor += 4;

    let num1 = parse_num(slice, cursor)?;
    if slice[*cursor] != ',' { 
        bail!("Could not find comma");
    }
    *cursor +=1;

    let num2 = parse_num(slice,cursor)?;

    if slice[*cursor] != ')' {
        bail!("Could not find closing paren");
    }
    *cursor += 1;

    Ok((num1, num2))
}

fn parse_num(slice: &[char], cursor: &mut usize) -> anyhow::Result<isize> {
    let mut char : char = slice[*cursor];

    if !char.is_digit(10) {
        anyhow::bail!("Not a number");
    }

    let mut num = 0;

    while char.is_digit(10) {
        num = num * 10 + char.to_digit(10).context("Could not conver char to digit")?;
        *cursor += 1;
        if *cursor >= slice.len() {
            bail!("Index out of bounds");

        }
        char = slice[*cursor];
    }

    Ok(isize::try_from(num)?)
}
