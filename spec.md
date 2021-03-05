# The complete PSI cheatsheet
PSI is designed to be simple enough to be learnt quickly.
It does this by removing weird behaviour like other languages have and stripping out complicated paradigms and concepts.

It's also designed to be complete enough to express yourself perfectly. It does this by combining several types together where other languages have several complicated data structures to try and express the same thing.

It takes inspiration from several languages, so if you've already learnt or read up about these languages then you'll be able to learn PSI quite quickly. Here's the list:

- Ruby - Ranges and `loop` statement
- Lua - Tables
- Python - `in`, `and` and `or` keywords
- C - Comments

## Comments
Comments are a way to annotate your code to make it more clear what is happening
You can place comments wherever you wish, in functions, in loops, at the end of lines etc...

They are completely ignored by the interpreter and do not affect the output of the code. You can even place code inside the comments and it'll get completely ignored.

### Comments on one line
Comments on one line look like this:

```
// This is a comment
```

They start with `//` and stop at the end of a line
You can place code above it, below it or before the `//` on the same line:

```
code here  
// Comment
```

```
// Comment
code here
```

```
code here // Comment
```

### Comments on multiple lines
Sometimes it's not ideal to have comments on just one line and you may wish to write in a large amount of text that can span several lines. This can be done like this:

```
/*
	This is a multiline comment
	It can span multiple lines
*/
```

They always start with `/*` and always end with `*/`.

You can also use these on a single line:

```
code here /* comment */ continue code here
```

```
/* this
works */
```

```
/*
this also works */
```

```
/* and this
*/
```

## Primitive datatypes
Primitive datatypes are the fundamental datatypes that make up a computer. They are very simple datatypes.

### Booleans
Booleans can be either `true` or `false`:

```
true
false
```

### Integers
Integers are whole numbers:
```
1
-4
7
-10
4953
``` 

They can be negative

### Floats
Floats are numbers with decimals

```
0.5
4.5
-4.0
385.716
```

They can be negative, just like integers

### Ranges
Not every language has a range datatype, PSI has one because they are incredibly useful for many purposes. There are two types of ranges.

#### Inclusive
These are given by the syntax `START...STOP` with three `.`s
They always increment by 1.

```
1...10 // This consists of numbers 1 to 10 (including 10)
```

You can't give floats here, but you can give negative numbers

```
-4...0 // This consists of numbers -4 to 0 (including 0)
```

You can also make ranges span downwards

```
-5...-10 // This consists of numbers -5 to -10 (including -10)
8...3    // This consists of numbers 8 to 3 (including 3)
```

#### Exclusive
These are given by the syntax `START..STOP` with two `.`s
They always increment by 1.

```
1..10 // This consists of numbers 1 to 9 (not including 10)
```

You can't give floats here, but you can give negative numbers

```
-4..0 // This consists of numbers -4 to -1 (not including 0)
```

You can also make ranges span downwards

```
-5..-10 // This consists of numbers -5 to -9 (not including -10)
8..3    // This consists of numbers 8 to 2 (not including 3)
```

### Nil
Nil represents nothing. It's probably the most controversial datatype, PSI wants to remove `nil` in favour of correct error handling, eventually.

## Arithmetic
Arithmetic with the 6 common operators are as follows:

```
4 + 7 // => 11  (addition)
6 - 2 // => 4   (subtraction)
4 * 1 // => 4   (multiplication)
5 / 2 // => 2.5 (division)
4 % 2 // => 0   (modulo)
3 ^ 2 // => 9   (exponentiation / power)
```

PSI adheres to BIDMAS / PEDMAS, you can use brackets to change the precedence

```
4 + 8 / 4   // => 6
(4 + 8) / 4 // => 3
```

## Equality and Comparison
To help compare datatypes, you can use these operators

```
4 == 5         // => false (is equal to)
3 == true      // => false
false == false // => true

4 != 4         // => false (is not equal to)
5 != 6         // => true
true != false  // => true

4 > 5          // => false (is greater than)
10.5 > -4      // => true
-6 > -6        // => false
3 < 2          // => false (is less than)
-5 < 1         // => true
5 < 5          // => false

4 >= 5         // => false (is greater than or equal to)
5 >= 5         // => true
4 <= 5         // => true (is less than or equal to)
5 <= 2         // => false
```

## Logic operations
You can combine equality and comparison together with several operations

### Not
This turns `true` to `false` and `false` to `true`. Just flips them around.

You can use the `not` or `!` syntax for this, they both do the same thing.

```
not true  // => false 
not false // => true
!true     // => false
!false    // => true
```

### And
This evaluates to true as long as the two sides are true, any other combination will evaluate to false

```
true and false  // => false
false and true  // => false
false and false // => false
true and true   // => true
```

### Or
This evaluates to true if either side (or both) is true

```
true or false  // => true
false or true  // => true
true or true   // => true
false or false // => false
```

## Strings
Strings are useful datatypes that allow storage of text. They are surrounded by quotes at the start and end: `"`.


```
"This is a string!"
```

### Concatenation
This is where you can add several strings together. For this you can use the `+` operator.

```
"Hello" + " world" // => "Hello world"
"Me" + "l" + "on"  // => "Melon"
```

### Interpolation
This is where you can put expressions inside strings with ease.

```
print("Five plus two is: {5 + 2}.") // => "Five plus two is 7."
```

Expressions are wrapped with `{` and `}`, in here you can put any expression you'd like (including variables, which are further next on the cheatsheet)

### Escaping
What if you want to put quotes in the string?

```
"Here's a quote: \" and it's inside a string!"
```

Quotes inside strings must have `\` before them to tell the language that you do not wish to end the string and instead just have a quote character.

### Indexing
A strings characters can be indexed like so:

```
"Hello World"[0] // => "H"
"Melon"[2]       // => "l"
```

Note that this language counts from 0, 0 is the first element.
You can also use negative numbers to get the characters from the back.

```
"Chicken"[-1] // => "n"
"Chicken"[-2] // => "e"
// Etc...
```

## Variables
Variables are like buckets of data, you can store anything you'd like here.

They can be declared and used like so:

```
name = "John"
age = 21

"My name is {name} and I'm {age} years old"
```

## Functions
Functions are a way to group together code that does specific things. They can be reused multiple times. Some functions are provided to you in the language such as `print` and `input`.

### Calling functions
The `print` function outputs things to the user, here's a classic example:

```
print("Hello World!")
```

### Defining functions

## Arrays

### Insertion

### Deletion

### Indexing

### Extending

### Contains

## Tables

### Insertion

### Deletion

### Indexing

### Extending

### Contains

## Selection

### If..Else

### Case

## Iteration

### Loop

### While

### For

## Table methods

### Defining methods

### Calling methods

## Libraries

### Internal

### External

## Standard library

### Filesystem

### Platform

### Shell commands

### Terminal