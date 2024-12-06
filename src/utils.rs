use std::io::Read;

pub fn get_input() -> anyhow::Result<String> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut stdin = std::io::stdin();
    stdin.read_to_end(&mut buffer)?;

    return Ok(String::from_utf8(buffer)?)
}
