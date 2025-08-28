use anyhow::Result;
use getrandom::getrandom;

fn main() -> Result<()> {
    let mut buf = [0u8; 4];
    getrandom(&mut buf)?;
    let random_u32 = u32::from_ne_bytes(buf);
    println!("{}", random_u32);
    Ok(())
}
