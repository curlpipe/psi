use psi::{do_parse, do_walk, Error, Interpreter};
use std::io::{self, Write};
use std::time::Instant;
use std::{env, fs};

fn main() -> Result<(), Error> {
    let mut args = env::args().skip(1);
    while let Some(argument) = args.next() {
        match argument.trim() {
            "-h" => help(),
            "repl" => repl(),
            "run" => {
                if let Some(path) = args.next() {
                    file(&path)
                } else {
                    help()
                }
            }
            _ => {
                println!("Error: {} isn't a valid argument", argument);
                Ok(())
            }
        }?
    }
    Ok(())
}

fn repl() -> Result<(), Error> {
    let mut interpreter = Interpreter::new();
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        let start_total = Instant::now();
        let parsed = do_parse(&input)?;
        println!("Parsing ΔΘ    | {:?}", Instant::now() - start_total);
        let start = Instant::now();
        let walked = do_walk(parsed)?;
        println!("Walking ΔΘ    | {:?}", Instant::now() - start);
        let start = Instant::now();
        interpreter.run_block(&walked)?;
        println!("Evaluation ΔΘ | {:?}", Instant::now() - start);
        println!("Total ΔΘ      | {:?}", Instant::now() - start_total);
        println!("{:?}", interpreter.env);
    }
}

fn file(path: &str) -> Result<(), Error> {
    let mut interpreter = Interpreter::new();
    let src = fs::read_to_string(path)?;
    let start_total = Instant::now();
    let parsed = do_parse(&src)?;
    println!("Parsing ΔΘ    | {:?}", Instant::now() - start_total);
    let start = Instant::now();
    let walked = do_walk(parsed)?;
    println!("Walking ΔΘ    | {:?}", Instant::now() - start);
    let start = Instant::now();
    interpreter.run_block(&walked)?;
    println!("Evaluation ΔΘ | {:?}", Instant::now() - start);
    println!("Total ΔΘ      | {:?}", Instant::now() - start_total);
    Ok(())
}

fn help() -> Result<(), Error> {
    println!(
        "
Ψ PSI Interpreter v{}
  Usage:
    {p} repl       : Opens a REPL
    {p} run [FILE] : Executes a file
  Flags:
    {p} -h         : Displays help message
",
        env!("CARGO_PKG_VERSION"),
        p = env::args().next().unwrap().as_str(),
    );
    std::process::exit(0);
}
