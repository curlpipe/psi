// Psibyte - A bytecode implementation of the PSI language
mod precedence;
mod compiler;
mod chunk;
mod error;
mod lexer;
mod value;
mod vm;

use precedence::{Precedence, get_rule};
use lexer::{Token, Lexer, TokenKind};
use chunk::{OpCode, Chunk};
use compiler::Compiler;
use std::time::Instant;
use clap::{App, Arg};
use scanln::scanln;
use error::Error;
use value::Value;
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
        println!("Error: Failed to find file '{}'", path);
    }
}

fn repl(verbose: bool) {
    // Initiate bytecode VM
    let mut vm = VM::new(verbose);
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
        println!("Lexing from character stream to token stream..."); 
    }
    let mut lexer = Lexer::new(&src);
    // Run the lexer and handle any errors
    if let Err(error) = lexer.run() {
        println!("> {}", error);
        return
    }
    // Show result
    if verbose { 
        println!("\nSuccess! Token stream:"); 
        lexer.display();
    }
    // Initiate compiler
    if verbose { 
        println!("\nCompiling from token stream to bytecode chunk..."); 
    }
    let mut compiler = Compiler::new(lexer.tokens);
    // Run the compiler and handle any errors
    if let Err(error) = compiler.compile() {
        println!("> {}", error);
        return
    }
    // Show result
    if verbose {
        println!("\nSuccess! Disassembled bytecode chunk:");
        compiler.display();
    }
    // Run virtual machine
    if verbose {
        println!("\nExecuting bytecode chunk in VM:")
    }
    if let Err(error) = vm.run(compiler.chunk) {
        println!("> {}", error);
        return
    }
    // Display success
    let end = Instant::now();
    if verbose { 
        println!("\nSuccess! Done in {:?}", end - start); 
    }
}
