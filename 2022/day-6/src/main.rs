use anyhow::Result;

const TOKEN_LEN: usize = 14;

fn main() -> Result<()> {
    let input: Vec<u8> = std::fs::read("input.txt")?;

    let pos = input.windows(TOKEN_LEN).position(|wind| {
        let mut seen = 0u32;
        for num in wind.iter() {
            let mask = 1 << (num - 97);
            if seen & mask != 0 {
                return false; 
            }
            seen |= mask;
        }
        true
    }).unwrap();

    println!("{}", pos + TOKEN_LEN);

    Ok(())
}
