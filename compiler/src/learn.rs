use lliw::{Style::Bold, Fg::{Green, Blue, Red}, Reset};
use psi_lang::{Compiler, Lexer, VM, Error};
use std::time::{Instant, Duration};
use std::io::{self, Write};
use std::thread::sleep;
use scanln::scanln;

const DELAY: u64 = 30;

macro_rules! sprintln {
    ($fmt:literal, $($args:expr),*) => {
        for c in format!($fmt, $( $args )*).chars() {
            print!("{}", c);
            io::stdout().flush().expect("Failed to flush stdout");
            sleep(Duration::from_millis(DELAY));
        }
        println!("");
    };
    ($fmt:literal) => {
        for c in $fmt.chars() {
            print!("{}", c);
            io::stdout().flush().expect("Failed to flush stdout");
            sleep(Duration::from_millis(DELAY));
        }
        println!("");
    };
}

fn run(check: &str) -> Result<(), Error> {
    let valid = check.to_ascii_lowercase().replace(" ", "").replace("\n", "");
    let src = scanln!("> ");
    if src.to_ascii_lowercase().replace(" ", "").replace("\n", "") != valid {
        sprintln!("Please try typing in {}", check);
        return Err(Error::UnexpectedEOI("Check failed".to_string()));
    }
    let start = Instant::now();
    let mut lexer = Lexer::new(&src);
    lexer.run()?;
    let mut compiler = Compiler::new(lexer.tokens);
    compiler.compile()?;
    let mut vm = VM::new(false);
    vm.run(compiler.chunk)?;
    let end = Instant::now();
    println!("{}{}Success!{} Done in {}{:?}{}", Green, Bold, Reset, Blue, end - start, Reset);
    Ok(())
}

fn run_error(check: &str) -> bool {
    let valid = check.to_ascii_lowercase().replace(" ", "").replace("\n", "");
    let src = scanln!("> ");
    if src.to_ascii_lowercase().replace(" ", "").replace("\n", "") != valid {
        sprintln!("Please try typing in {}", check);
        return true
    }
    let mut lexer = Lexer::new(&src);
    if let Err(error) = lexer.run() {
        error.display_line(&src);
        println!("{}{}{}{}", Red, Bold, error, Reset);
        return false
    }
    let mut compiler = Compiler::new(lexer.tokens);
    if let Err(error) = compiler.compile() {
        error.display_line(&src);
        println!("{}{}{}{}", Red, Bold, error, Reset);
        return false
    }
    let mut vm = VM::new(false);
    if let Err(error) = vm.run(compiler.chunk) {
        error.display_line(&src);
        println!("{}{}{}{}", Red, Bold, error, Reset);
        vm.reset();
        return false
    }
    false
}

pub fn learn() {
    // Introduction
    sprintln!("Hey! Thanks for wanting to give psi a try!");
    let name = scanln!("What can I call you? ");
    sprintln!("\nWelcome to the interactive learning environment, {}", name);
    sprintln!("It's nice to meet you.");
    sprintln!("\nHere is a repl (read-evaluate-print-loop)");
    sprintln!("> ");
    sprintln!("\nThis is where you execute code");
    sprintln!("You can type out your code and then press enter to execute it");
    // Arithmetic
    sprintln!("\nLet's start off with performing some arithmetic!");
    sprintln!("Start out with a simple `1 + 1`");
    while run("1 + 1").is_err() {  }
    sprintln!("Great work! Notice the result comes back as 2, pretty simple");
    sprintln!("There are also other operators, try out the subtraction operator `4 - 2`");
    while run("4 - 2").is_err() {  }
    sprintln!("Nice! Once again, we can see a 2");
    sprintln!("There are also divide and multiply operators");
    sprintln!("We can create more complicated arithmetic by combining operations `4 * 2 / 6`");
    while run("4 * 2 / 6").is_err() {  }
    sprintln!("Brilliant! 1.333333333...");
    sprintln!("PSI adheres to the BIDMAS precedence rules.");
    sprintln!("( PEMDAS for you 'muricans :D )");
    sprintln!("You can use brackets to raise the precedence, try `1 + 2 / 3`");
    while run("1 + 2 / 3").is_err() {  }
    sprintln!("now try `(1 + 2) / 3`");
    while run("(1 + 2) / 3").is_err() {  }
    sprintln!("See how they evaluate to different things?");
    sprintln!("\nWhat about negative numbers?");
    sprintln!("You can use negative numbers too, try out `-3 + 4`");
    while run("-3 + 4").is_err() {  }
    sprintln!("Excellent! this gives back 1");
    sprintln!("Negation has a high precedence, try out `-(3 + 4)`");
    while run("-(3 + 4)").is_err() {  }
    sprintln!("Excellent! this gives back -7 due to the different order of operations");
    sprintln!("\nThere are two more arithmetic operators in PSI. The power operator `3 ^ 2`");
    while run("3 ^ 2").is_err() {  }
    sprintln!("Three squared is nine, handy");
    sprintln!("Finally there's the modulo operator, for the remainder `1 % 3`");
    while run("1 % 3").is_err() {  }
    sprintln!("1 / 3 gives a remainder of 1");
    // Equality
    sprintln!("This is quite useful for finding if a number is even `4 % 2 == 0`");
    while run("4 % 2 == 0").is_err() {  }
    sprintln!("Wait! What do these `==` mean?");
    sprintln!("They mean 'is equal to'");
    sprintln!("\nFirst PSI evaluates 4 % 2 (which evalutes to `0` if 4 is even)");
    sprintln!("Then we use the `==` operator to check if it's actually equal to 0");
    sprintln!("This will return either true or false. These are booleans");
    sprintln!("\nTrue means that 4 is even, false means 4 isn't even");
    sprintln!("In this case, 4 is even");
    sprintln!("therefore `4 % 2` gives 0 and `4 % 2 == 0` gives us true");
    sprintln!("Here's another example: `5 % 2 == 0` (is 5 even?) this would return false");
    sprintln!("\n`==` can be used for anything really, try `3 == 2`");
    while run("3 == 2").is_err() {  }
    sprintln!("Awesome! You see this evaluates to false, because 3 isn't equal to 2");
    sprintln!("\n`==` can be used regardless of datatypes `true == 3`");
    while run("true == 3").is_err() {  }
    sprintln!("Let's move onto our next operator: `!=`");
    sprintln!("`!=` means `is not equal to`");
    sprintln!("\ngive `5.7 != 3.2` a whirl");
    while run("5.7 != 3.2").is_err() {  }
    sprintln!("This returns true, because 5.7 isn't equal to 3.2");
    sprintln!("Notice the usage of floats here, floats are just decimal numbers");
    sprintln!("\nLet's learn about another useful operation: `not true`");
    sprintln!("not turns true into false and false into true, it just flips them");
    while run("not true").is_err() {  }
    sprintln!("This returns false, because what isn't true, is false");
    sprintln!("You can also use `!` to serve the same purpose, it does exactly the same thing");
    sprintln!("`!false`");
    while run("!false").is_err() {  }
    sprintln!("This returns true, because what isn't false, is true");
    // Comparison
    sprintln!("\nSo you've covered arithmetic, basic datatypes and equality");
    sprintln!("Great work! Next up is comparison");
    sprintln!("\n`>` means `is greater than`");
    sprintln!("Try out `5 > 2`");
    while run("5 > 2").is_err() {  }
    sprintln!("5 is greater than 2, therefore it returns true");
    sprintln!("\n`<` means `is less than`");
    sprintln!("Try out `5.4 < 5.3`");
    while run("5.4 < 5.3").is_err() {  }
    sprintln!("This gives back false because 5.4 is greater than 5.3, not less than");
    sprintln!("\n`>=` means `is greater than or equal to`");
    sprintln!("Try out `137 >= 137`");
    while run("137 >= 137").is_err() {  }
    sprintln!("137 is equal to 137, therefore it returns true");
    sprintln!("\n`<=` means `is less than`");
    sprintln!("Try out `3.141 <= 3`");
    while run("3.141 <= 3").is_err() {  }
    sprintln!("This gives back false because 3.141 is not equal to 3");
    sprintln!("3.141 is also greater than 3, not less than.");
    sprintln!("\nThat's comparison done, congratulations!");
    sprintln!("You can now use PSI as a desktop calculator :p");
    // Strings
    sprintln!("\nLet's move onto another datatype, strings.");
    sprintln!("Strings are a very important datatype, they allow us to store text");
    sprintln!("Strings are always surrounded by quotes: `\"Hello World!\"`");
    while run("\"Hello World!\"").is_err() {  }
    sprintln!("What about joining strings together?");
    sprintln!("We can use a process called `concatenation`");
    sprintln!("It's just like adding two numbers together: `\"Me\" + \"lon\"`");
    while run("\"Me\" + \"lon\"").is_err() {  }
    sprintln!("Great!");
    // Comments
    sprintln!("\nSometimes all this code can be a bit confusing as to what it's doing");
    sprintln!("Wouldn't it be great if programmers could annotate their code?");
    sprintln!("Most languages have something called `comments`, including PSI");
    sprintln!("Comments are completely ignored by the language, you can put anything in them");
    sprintln!("There are both single-line and multi-line comments");
    sprintln!("Here's a single line comment: `// This is a comment`");
    while run("// This is a comment").is_err() {  }
    sprintln!("You can use it in the same line as code: `5 % 2 == 0 // Is 5 even?`");
    while run("5 % 2 == 0 // Is 5 even?").is_err() {  }
    sprintln!("Aha! The comment is completely ignored and only the code is run");
    sprintln!("Single line comments run to the end of the line and then stop");
    sprintln!("Next up is the multiline comment:");
    sprintln!("```\n/*\nHello \nWorld!\n*/\n```");
    sprintln!("These start with `/*` and end with `*/`");
    sprintln!("These comments can be embedded inbetween code: `5 /* comment */ + 2`");
    while run("5 /* comment */ + 2").is_err() {  }
    sprintln!("This will just evaluate to 7, ignoring the comment");
    sprintln!("These comments can span multiple lines too, this is useful if you want to");
    sprintln!("insert a lot of text into your code");
    sprintln!("on this repl, you can't create newlines unfortunately, but when you write");
    sprintln!("code in a file, you'll be able to use these");
    // Errors
    sprintln!("\nAll this sucessful code is making me bored :p");
    sprintln!("Time to try and trick and confuse PSI with invalid code");
    sprintln!("Let's try out some weird things like this: `\"string`");
    sprintln!("Here there is no ending quote on the string, this is going to cause problems");
    while run_error("\"string") {  }
    sprintln!("These errors show you the exact problem with the code, brilliant");
    sprintln!("Let's try out some weird things like this: `1 + +`");
    sprintln!("Here we provided a + instead of a datatype, this will cause problems");
    while run_error("1 + + ") {  }
    sprintln!("`true - false`");
    sprintln!("Here we are trying to take a boolean away from a boolean, which is impossible");
    while run_error("true - false") {  }
    sprintln!("`3 & 2`");
    sprintln!("Here we have a '&' symbol there, which is invalid");
    while run_error("3 & 2") {  }
    sprintln!("`(4 + 2`");
    sprintln!("Here we didn't close the bracket");
    while run_error("(4 + 2") {  }
    sprintln!("`2 + `");
    sprintln!("Here we failed to provide a right hand operand");
    while run_error("2 + ") {  }
    sprintln!("`true + 3`");
    sprintln!("Here we are adding two incompatible datatypes");
    while run_error("true + 3") {  }
    sprintln!("`\"Hi\" - \"Hi\"`");
    sprintln!("Here we are trying to take strings away from each other, impossible!");
    while run_error("\"Hi\" - \"Hi\"") {  }
    sprintln!("`-true`");
    sprintln!("Here we are trying to create a negative boolean, which doesn't exist");
    while run_error("-true") {  }
    sprintln!("`not 3`");
    sprintln!("Here we are trying to not a number, which is impossible");
    while run_error("not 3") {  }
    sprintln!("That brings us to the end of the tour, hope you enjoyed it!");
    sprintln!("You are now up to date with PSI, if you wish to play around");
    sprintln!("a bit more freely, you can run `psi -r` to access a repl");
    sprintln!("where you can type anything you wish in there.");
    sprintln!("Goodbye, {}, it was nice to meet you!", name);
}
