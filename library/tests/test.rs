// Test.rs - Test suite for the bytecode implementation of psi
#![allow(clippy::approx_constant)]
#[cfg(test)]
use psi_lang::{Lexer, TokenKind::*, Token, Error, Compiler, Chunk, OpCode::*, Value, VM};

/*
    This is a test for the Lexer
    A lexer takes care of:
        - Parsing datatypes like numbers and strings
        - Providing column, line and other position information
        - Strip whitespace and comments
        - Prevent unknown characters from entering into the compiler
    It turns a character stream, the raw text that the user has written
    into a list of "tokens" which represent the meaning of that text
    e.g. "4 + 6" is [NUMBER(4), PLUS, NUMBER(6)]
    This is the first stage in most interpreters and compilers
*/
#[test]
fn lexer() {
    // Test the Lexer at arithmetic
    let mut lexer = Lexer::new("1 + 1 - 45 / 21 * 534 ^ (3897 % 4)");
    assert!(lexer.run().is_ok());
    assert_eq!(lexer.tokens, [
        Token { kind: Number(1.0), start: 0, len: 1, line: 1, col: 1 }, 
        Token { kind: Plus, start: 2, len: 1, line: 1, col: 3 }, 
        Token { kind: Number(1.0), start: 4, len: 1, line: 1, col: 5 }, 
        Token { kind: Minus, start: 6, len: 1, line: 1, col: 7 }, 
        Token { kind: Number(45.0), start: 8, len: 2, line: 1, col: 9 }, 
        Token { kind: Slash, start: 11, len: 1, line: 1, col: 12 }, 
        Token { kind: Number(21.0), start: 13, len: 2, line: 1, col: 14 }, 
        Token { kind: Asterisk, start: 16, len: 1, line: 1, col: 17 }, 
        Token { kind: Number(534.0), start: 18, len: 3, line: 1, col: 19 }, 
        Token { kind: Hat, start: 22, len: 1, line: 1, col: 23 }, 
        Token { kind: LeftParen, start: 24, len: 1, line: 1, col: 25 },
        Token { kind: Number(3897.0), start: 25, len: 4, line: 1, col: 26 }, 
        Token { kind: Percent, start: 30, len: 1, line: 1, col: 31 }, 
        Token { kind: Number(4.0), start: 32, len: 1, line: 1, col: 33 }, 
        Token { kind: RightParen, start: 33, len: 1, line: 1, col: 34 },
        Token { kind: EOI, start: 34, len: 0, line: 1, col: 35 },
    ]);
    // Test the lexer at other operators & datastructures & comments
    let mut lexer = Lexer::new("true == \"Hello World\" != false // Hello\n");
    assert!(lexer.run().is_ok());
    assert_eq!(lexer.tokens, [
        Token { kind: True, start: 0, len: 4, line: 1, col: 1 }, 
        Token { kind: Equals, start: 5, len: 2, line: 1, col: 6 }, 
        Token { kind: String("Hello World".to_string()), start: 8, len: 13, line: 1, col: 9 }, 
        Token { kind: NotEquals, start: 22, len: 2, line: 1, col: 23 }, 
        Token { kind: False, start: 25, len: 5, line: 1, col: 26 }, 
        Token { kind: Comment, start: 31, len: 8, line: 1, col: 32 }, 
        Token { kind: EOI, start: 40, len: 0, line: 2, col: 2 }
    ]);
    // Test the lexer at more operators & datastructures & comments
    let mut lexer = Lexer::new("4 > 3 < 2 >= 5 <= 3 == !true");
    assert!(lexer.run().is_ok());
    assert_eq!(lexer.tokens, [
        Token { kind: Number(4.0), start: 0, len: 1, line: 1, col: 1 }, 
        Token { kind: Greater, start: 2, len: 1, line: 1, col: 3 }, 
        Token { kind: Number(3.0), start: 4, len: 1, line: 1, col: 5 }, 
        Token { kind: Less, start: 6, len: 1, line: 1, col: 7 }, 
        Token { kind: Number(2.0), start: 8, len: 1, line: 1, col: 9 }, 
        Token { kind: GreaterEq, start: 10, len: 2, line: 1, col: 11 }, 
        Token { kind: Number(5.0), start: 13, len: 1, line: 1, col: 14 }, 
        Token { kind: LessEq, start: 15, len: 2, line: 1, col: 16 }, 
        Token { kind: Number(3.0), start: 18, len: 1, line: 1, col: 19 }, 
        Token { kind: Equals, start: 20, len: 2, line: 1, col: 21 }, 
        Token { kind: Exclamation, start: 23, len: 1, line: 1, col: 24 }, 
        Token { kind: True, start: 24, len: 4, line: 1, col: 25 }, 
        Token { kind: EOI, start: 28, len: 0, line: 1, col: 29 }
    ]);
    // Test the lexer at unicode & newlines & multiline comments
    let mut lexer = Lexer::new("\"H 你好 hi\"\n\n/* hello\nthere*/\t\n");
    assert!(lexer.run().is_ok());
    assert_eq!(lexer.tokens, [
        Token { kind: String("H 你好 hi".to_string()), start: 0, len: 9, line: 1, col: 1 },
        Token { kind: Comment, start: 11, len: 16, line: 3, col: 2 }, 
        Token { kind: EOI, start: 29, len: 0, line: 5, col: 2 },
    ]);
    // Multiline strings & floats & unary operations (with nil)
    let mut lexer = Lexer::new("\"hello\nworld\" == 3.141 != not nil");
    assert!(lexer.run().is_ok());
    assert_eq!(lexer.tokens, [
        Token { kind: String("hello\nworld".to_string()), start: 0, len: 13, line: 1, col: 1 }, 
        Token { kind: Equals, start: 14, len: 2, line: 2, col: 9 }, 
        Token { kind: Number(3.141), start: 17, len: 5, line: 2, col: 12 }, 
        Token { kind: NotEquals, start: 23, len: 2, line: 2, col: 18 }, 
        Token { kind: Not, start: 26, len: 3, line: 2, col: 21 }, 
        Token { kind: Nil, start: 30, len: 3, line: 2, col: 25 }, 
        Token { kind: EOI, start: 33, len: 0, line: 2, col: 28 },
    ]);
    // Errors & formatting
    let mut lexer = Lexer::new("234786 你");
    assert_eq!(lexer.run(), Err(Error::UnexpectedCharacter('你', 1, 8, 1)));
    let mut lexer = Lexer::new("note");
    assert_eq!(lexer.run(), Err(Error::UnexpectedCharacter('n', 1, 1, 1)));
    // Displaying of tokens
    let mut lexer = Lexer::new("2 + 3");
    assert!(lexer.run().is_ok());
    lexer.display();
}

/*
    This is a test for the Compiler
    A compiler takes care of:
        - Working out the precedence of operations
        - Working out what instructions to use in what order
        - Emitting valid bytecode for the VM to process
        - Preventing incorrect sequences of tokens e.g. "4 + +"
    It turns a token stream, the output of the lexer into bytecode
    which is a list of low-level instructions for operating on the data
    e.g. [NUMBER(4), PLUS, NUMBER(6)] generates [OP_CONSTANT(0), OP_CONSTANT(1), OP_ADD]
    This is the second stage and works out how tokens relate to each other
    It is probably one of the most complicated parts of this program
*/
#[test]
fn compiler() {
    // Test arithmetic precedence and operations
    let mut lexer = Lexer::new("(1 + 2) / 3 - 4 * 5 % 6 ^ 7");
    assert!(lexer.run().is_ok());
    let mut compiler = Compiler::new(lexer.tokens);
    assert!(compiler.compile().is_ok());
    assert_eq!(compiler.chunk, Chunk {
        code: vec![
            (2, 1, OpConstant(0)), (6, 1, OpConstant(1)), (4, 1, OpAdd), 
            (11, 1, OpConstant(2)), (9, 1, OpDiv), 
            (15, 1, OpConstant(3)), (19, 1, OpConstant(4)), (17, 1, OpMul), 
            (23, 1, OpConstant(5)), (27, 1, OpConstant(6)), (25, 1, OpPow), 
            (21, 1, OpMod), (13, 1, OpSub), (28, 0, OpReturn)
        ],
        constants: vec![
            Value::Number(1.0), Value::Number(2.0), 
            Value::Number(3.0), Value::Number(4.0), 
            Value::Number(5.0), Value::Number(6.0), 
            Value::Number(7.0),
        ],
        line: 1,
    });
    // Test comparison
    let mut lexer = Lexer::new("(4 + 23 > 324 == 32 <= 1) != (5 - 3 < 3 ^ 5 == 3 >= 4)");
    assert!(lexer.run().is_ok());
    let mut compiler = Compiler::new(lexer.tokens);
    assert!(compiler.compile().is_ok());
    assert_eq!(compiler.chunk, Chunk {
        code: vec![
            (2, 1, OpConstant(0)), (6, 2, OpConstant(1)), (4, 1, OpAdd), 
            (11, 3, OpConstant(2)), (9, 1, OpGreater), 
            (18, 2, OpConstant(3)), (24, 1, OpConstant(4)), (21, 2, OpGreater), 
            (21, 2, OpNot), (15, 2, OpEqual), 
            (31, 1, OpConstant(5)), (35, 1, OpConstant(6)), (33, 1, OpSub), 
            (39, 1, OpConstant(7)), (43, 1, OpConstant(8)), (41, 1, OpPow), 
            (37, 1, OpLess), 
            (48, 1, OpConstant(9)), (53, 1, OpConstant(10)), (50, 2, OpLess), 
            (50, 2, OpNot), (45, 2, OpEqual), (27, 2, OpEqual), (27, 2, OpNot), 
            (55, 0, OpReturn)
        ],
        constants: vec![
            Value::Number(4.0), Value::Number(23.0), 
            Value::Number(324.0), Value::Number(32.0),
            Value::Number(1.0), Value::Number(5.0),
            Value::Number(3.0), Value::Number(3.0),
            Value::Number(5.0), Value::Number(3.0),
            Value::Number(4.0),
        ],
        line: 1,
    });
    // Test equality
    let mut lexer = Lexer::new("(true == nil) != (\"Hello\" == 4 + 6 ^ 2)");
    assert!(lexer.run().is_ok());
    let mut compiler = Compiler::new(lexer.tokens);
    assert!(compiler.compile().is_ok());
    assert_eq!(compiler.chunk, Chunk {
        code: vec![
            (2, 4, OpTrue), (10, 3, OpNil), (7, 2, OpEqual), 
            (19, 7, OpConstant(0)), (30, 1, OpConstant(1)), 
            (34, 1, OpConstant(2)), (38, 1, OpConstant(3)), (36, 1, OpPow), 
            (32, 1, OpAdd), (27, 2, OpEqual), (15, 2, OpEqual), (15, 2, OpNot), 
            (40, 0, OpReturn)
        ],
        constants: vec![
            Value::String("Hello".to_string()), Value::Number(4.0), 
            Value::Number(6.0), Value::Number(2.0),
        ],
        line: 1,
    });
    // Test unary operations
    let mut lexer = Lexer::new("!true == not true");
    assert!(lexer.run().is_ok());
    let mut compiler = Compiler::new(lexer.tokens);
    assert!(compiler.compile().is_ok());
    assert_eq!(compiler.chunk, Chunk {
        code: vec![
            (2, 4, OpTrue), (1, 1, OpNot), 
            (14, 4, OpTrue), (10, 3, OpNot), 
            (7, 2, OpEqual), (18, 0, OpReturn)
        ],
        constants: vec![],
        line: 1,
    });
    // Test comment jumping
    let mut lexer = Lexer::new("/* haha */ 1 + /* hello */ 2 / 37 // Lol");
    assert!(lexer.run().is_ok());
    let mut compiler = Compiler::new(lexer.tokens);
    assert!(compiler.compile().is_ok());
    assert_eq!(compiler.chunk, Chunk {
        code: vec![
            (12, 1, OpConstant(0)), (28, 1, OpConstant(1)), 
            (32, 2, OpConstant(2)), (30, 1, OpDiv), 
            (14, 1, OpAdd), (42, 0, OpReturn)
        ],
        constants: vec![
            Value::Number(1.0), Value::Number(2.0),
            Value::Number(37.0),
        ],
        line: 1,
    });
    // Test empty comment lines
    let mut lexer = Lexer::new("// Lol");
    assert!(lexer.run().is_ok());
    let mut compiler = Compiler::new(lexer.tokens);
    assert!(compiler.compile().is_ok());
    assert_eq!(compiler.chunk, Chunk {
        code: vec![],
        constants: vec![],
        line: 1,
    });
    // Test displaying
    let mut lexer = Lexer::new("1 + 2 / 3 - 4 * 5 % 6 ^ 7");
    assert!(lexer.run().is_ok());
    let mut compiler = Compiler::new(lexer.tokens);
    assert!(compiler.compile().is_ok());
    compiler.display();
    // Test errors
    let mut lexer = Lexer::new("1 + +");
    assert!(lexer.run().is_ok());
    let mut compiler = Compiler::new(lexer.tokens);
    assert_eq!(compiler.compile(), Err(Error::ExpectedExpression(1, 5, 1)));
    let mut lexer = Lexer::new("(1 + 3");
    assert!(lexer.run().is_ok());
    let mut compiler = Compiler::new(lexer.tokens);
    assert_eq!(compiler.compile(), Err(Error::ExpectedToken(RightParen, 1, 7, 0)));
}

/*
    This is a test for the virtual machine
    A VM takes care of:
        - Evaluating bytecode using a stack
        - Erroring correctly on runtime errors (e.g. "true - false")
        - Showing output at the end of execution
    It turns bytecode into an output.
    e.g. [OP_CONSTANT(0), OP_CONSTANT(1), OP_ADD], [4, 6] becomes 10
    This is the third and final stage and usually takes the longest time
*/
#[test]
fn virtual_machine() {
    // Test arithmetic & negation
    let mut lexer = Lexer::new("(1 + 2) / 3 - 4 * -5 % 6 ^ -7");
    assert!(lexer.run().is_ok());
    let mut compiler = Compiler::new(lexer.tokens);
    assert!(compiler.compile().is_ok());
    let mut vm = VM::new(true);
    assert!(vm.run(compiler.chunk).is_ok());
    assert_eq!(vm.result, Some(Value::Number(1.0)));
    // Test string concatenation
    let mut lexer = Lexer::new("\"Hello\" + \" World!\"");
    assert!(lexer.run().is_ok());
    let mut compiler = Compiler::new(lexer.tokens);
    assert!(compiler.compile().is_ok());
    let mut vm = VM::new(true);
    assert!(vm.run(compiler.chunk).is_ok());
    assert_eq!(vm.result, Some(Value::String("Hello World!".to_string())));
    // Test unary & equality
    let mut lexer = Lexer::new("not true == nil");
    assert!(lexer.run().is_ok());
    let mut compiler = Compiler::new(lexer.tokens);
    assert!(compiler.compile().is_ok());
    let mut vm = VM::new(false);
    assert!(vm.run(compiler.chunk).is_ok());
    assert_eq!(vm.result, Some(Value::Boolean(false)));
    let mut lexer = Lexer::new("(1 != 2) == !false");
    assert!(lexer.run().is_ok());
    let mut compiler = Compiler::new(lexer.tokens);
    assert!(compiler.compile().is_ok());
    let mut vm = VM::new(true);
    assert!(vm.run(compiler.chunk).is_ok());
    assert_eq!(vm.result, Some(Value::Boolean(true)));
    // Test comparison
    let mut lexer = Lexer::new("(2 < 5) == (4 > 6)");
    assert!(lexer.run().is_ok());
    let mut compiler = Compiler::new(lexer.tokens);
    assert!(compiler.compile().is_ok());
    let mut vm = VM::new(false);
    assert!(vm.run(compiler.chunk).is_ok());
    assert_eq!(vm.result, Some(Value::Boolean(false)));
    let mut lexer = Lexer::new("(2 <= 2) == (6 >= 4)");
    assert!(lexer.run().is_ok());
    let mut compiler = Compiler::new(lexer.tokens);
    assert!(compiler.compile().is_ok());
    let mut vm = VM::new(true);
    assert!(vm.run(compiler.chunk).is_ok());
    assert_eq!(vm.result, Some(Value::Boolean(true)));
    // Test errors
    let mut lexer = Lexer::new("!3 - 3");
    assert!(lexer.run().is_ok());
    let mut compiler = Compiler::new(lexer.tokens);
    assert!(compiler.compile().is_ok());
    let mut vm = VM::new(true);
    assert_eq!(
        vm.run(compiler.chunk), 
        Err(Error::MismatchedTypes(1, 2, 1, "Operand must be a boolean".to_string()))
    );
    let mut lexer = Lexer::new("3 - -true");
    assert!(lexer.run().is_ok());
    let mut compiler = Compiler::new(lexer.tokens);
    assert!(compiler.compile().is_ok());
    let mut vm = VM::new(true);
    assert_eq!(
        vm.run(compiler.chunk), 
        Err(Error::MismatchedTypes(1, 6, 4, "Operand must be a number".to_string()))
    );
    let mut lexer = Lexer::new("nil + nil");
    assert!(lexer.run().is_ok());
    let mut compiler = Compiler::new(lexer.tokens);
    assert!(compiler.compile().is_ok());
    let mut vm = VM::new(true);
    assert_eq!(
        vm.run(compiler.chunk), 
        Err(Error::MismatchedTypes(1, 1, 3, "Operands must be either numbers or strings".to_string()))
    );
    let mut lexer = Lexer::new("\"Impossible\" - \"Operation\"");
    assert!(lexer.run().is_ok());
    let mut compiler = Compiler::new(lexer.tokens);
    assert!(compiler.compile().is_ok());
    let mut vm = VM::new(true);
    assert_eq!(
        vm.run(compiler.chunk), 
        Err(Error::ImpossibleOperation(1, 14, 1, "-".to_string()))
    );
    // Test VM resetting
    vm.reset();
    assert!(vm.stack.is_empty());
    assert!(vm.positions.is_empty());
}

/*
    This is a test for the error reporting system
    This takes care of:
        - Reporting the correct location of the error
        - Clearly showing the programmer what went wrong where
        - Being able to highlight the problem area
        - Represent every single failure point
    This is a fundamental part of the language and 
    ironically, the error reporting code is known to panic
*/
#[test]
fn errors() {
    let string = "4 & 2";
    let character = Error::UnexpectedCharacter('&', 1, 3, 1);
    character.display_line(string);
    let string = "\"hello";
    let eoi = Error::UnexpectedEOI("Unterminated string".to_string());
    eoi.display_line(string);
    let string = "(4 + 2";
    let token = Error::ExpectedToken(RightParen, 1, 7, 0);
    token.display_line(string);
    let string = "4 +";
    let expression = Error::ExpectedExpression(1, 4, 0);
    expression.display_line(string);
    let string = "true + 3";
    let types = Error::MismatchedTypes(1, 1, 4, "Operands must be numbers".to_string());
    types.display_line(string);
    let string = "\"a\" - \"b\"";
    let impossible = Error::ImpossibleOperation(1, 5, 1, "-".to_string());
    impossible.display_line(string);
}
