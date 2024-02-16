use std::{
    env,
    fs::File,
    io::{self, BufReader, Read},
    process,
    str::Chars,
};

mod scanner;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let path = &args[1];
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        let tokens = scanner::scan(&mut contents.chars())?;

        println!("{:?}", tokens);
    } else {
        println!("Usage: aqa-interpreter <file>");
        process::exit(-1);
    }

    Ok(())
}
