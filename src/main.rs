use rand;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let blocks = lear::blocks_to_show(&mut rng)?;
    // for block in blocks {
    lear::display(&blocks);
    // }
    Ok(())
}
