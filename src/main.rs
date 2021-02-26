// Psibyte - A bytecode implementation of the PSI language
mod precedence;
mod compiler;
mod chunk;
mod error;
mod lexer;
mod value;
mod vm;

use lliw::Fg::{Red, Yellow, Green, Blue};
use precedence::{Precedence, get_rule};
use lexer::{Token, Lexer, TokenKind};
use lliw::Style::{Bold, NoBold};
use chunk::{OpCode, Chunk};
use compiler::Compiler;
use std::time::Instant;
use clap::{App, Arg};
use scanln::scanln;
use error::Error;
use value::Value;
use lliw::Reset;
use std::fs;
use vm::VM;

fn main() {
    // Command line argument parser
    let args = App::new("PSI")
       .version(env!("CARGO_PKG_VERSION"))
       .about("A bytecode interpreter implementation for the PSI language")
       .arg(Arg::with_name("verbose")
           .short("v")
           .long("verbose")
           .takes_value(false)
           .help("Shows the internal workings of the interpreter"))
       .arg(Arg::with_name("file")
           .allow_hyphen_values(false)
           .required_unless("repl")
           .takes_value(true))
       .arg(Arg::with_name("repl")
           .long("repl")
           .short("r")
           .help("Access a read-evaluate-print-loop for trying out the language")
           .required_unless("file")
           .takes_value(false))
       .get_matches(); 

    // Handle command line arguments
    let verbose = args.is_present("verbose");
    if args.is_present("repl") {
        repl(verbose)
    } else if let Some(path) = args.value_of("file") {
        file(path, verbose)
    }
}

fn file(path: &str, verbose: bool) {
    // Read in file
    if let Ok(contents) = fs::read_to_string(path) {
        // Execute file contents
        let mut vm = VM::new(verbose);
        run(&contents, &mut vm, verbose)
    } else {
        println!("{}{}Error: Failed to find file '{}'{}", Red, Bold, path, Reset);
    }
}

fn repl(verbose: bool) {
    // Initiate bytecode VM
    let mut vm = VM::new(verbose);
    println!(
        "{}Ψ PSI Interpreter {}{}{}{}", 
        Bold, NoBold, 
        Blue, env!("CARGO_PKG_VERSION"), Reset
    );
    loop {
        // Prompt user for input
        let input: String = scanln!("> ");
        run(&input, &mut vm, verbose)
    }
}

fn run(src: &str, vm: &mut VM, verbose: bool) {
    // Start timer
    let start = Instant::now();
    // Initiate a lexer
    if verbose { 
        println!("{}{}Lexing from character stream to token stream...{}", Yellow, Bold, Reset);
    }
    let mut lexer = Lexer::new(&src);
    // Run the lexer and handle any errors
    if let Err(error) = lexer.run() {
        println!("=> {}{}{}{}", Red, Bold, error, Reset);
        return
    }
    // Show result
    if verbose { 
        println!("\n{}{}Success!{} Token stream:", Green, Bold, Reset); 
        lexer.display();
    }
    // Initiate compiler
    if verbose { 
        println!("\n{}{}Compiling from token stream to bytecode...{}", Yellow, Bold, Reset); 
    }
    let mut compiler = Compiler::new(lexer.tokens);
    // Run the compiler and handle any errors
    if let Err(error) = compiler.compile() {
        println!("=> {}{}{}{}", Red, Bold, error, Reset);
        return
    }
    // Show result
    if verbose {
        println!("\n{}{}Success!{} Disassembled bytecode:", Green, Bold, Reset);
        compiler.display();
    }
    // Run virtual machine
    if verbose {
        println!("{}{}\nExecuting bytecode chunk in VM:{}", Yellow, Bold, Reset);
    }
    if let Err(error) = vm.run(compiler.chunk) {
        vm.stack.clear(); // Clear the VM stack after a runtime error
        println!("=> {}{}{}{}", Red, Bold, error, Reset);
        return
    }
    // Display success
    let end = Instant::now();
    println!("{}{}Success!{} Done in {}{:?}{}", Green, Bold, Reset, Blue, end - start, Reset);
}
