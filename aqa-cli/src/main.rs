use std::{
    env,
    fs::File,
    io::{BufReader, Read},
};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let path = &args[1];
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        aqa::run(contents)?;
    } else {
        println!("Usage: aqa-cli <file>");
    }
    Ok(())
}
