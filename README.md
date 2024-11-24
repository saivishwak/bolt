# Bolt

Bolt is a scripting language designed to delve deep into the internals of programming languages. Its dynamic nature allows for an exploration of language concepts, with the lexer, parser, and evaluator all crafted from scratch. Developed using Rust as the host language, Bolt aims to incorporate advanced features such as asynchronous executions for blazing-fast performance in the future.

While not intended for production use at this stage, ongoing research will expand Bolt's capabilities. The project draws heavy inspiration from "Writing An Interpreter In Go" by Thorsten Ball (https://interpreterbook.com) and "Crafting Interpreters" by Robert Nystorm (https://craftinginterpreters.com), both of which offer invaluable insights into compiler construction and language design.

I extend my gratitude to the authors of these seminal works for their invaluable contributions to the field of interpreters and programming languages. I encourage interested individuals to explore these resources further for a deeper understanding of language implementation.

# Bolt specification

## High level features

- Dynamic typing
- Funtion executions
- Control Flow

## Installation
```sh
cargo build --features=llvm-sys/prefer-dynamic
cargo run --bin bolt_cli --features=llvm-sys/prefer-dynamic start
```

## Data Types

- Boolean - true, false
- Number - integer or decimal

```
let a = 10;
//Or
let b = 10.2;
```

Bolt only has one type which is Number and handles decimals and integers both as double-precision floating point

- String - "Hello world"
- NULL - No value

## Comparison and equality

less < than;
lessThan <= orEqual;
greater > than;
greaterThan >= orEqual;

```
1 == 2; // false.
"a" != "b"; // true.
```

Different types can be compared.

```
1 == "a"; // false.
```

Values of different types are never equivalent in Bolt.

```
123 == "123"; // false.
```

## Logical operators

The not operator, a prefix !, returns false if its operand is true, and vice
versa similar to Javascript.

```
!true; // false.
!false; // true.
```

## Control Flow

```if (condition) {
print "yes";
} else {
print "no";
}
```

#### Instructions

Install LLVM and Clang

```shell
git clone https://github.com/saivishwak/bolt.git
cd bolt
```

Using REPL

```shell
cargo run --bin bolt_cli start
```

Using interpretter

```shell
cargo run --bin bolt_cli run --path=./examples/hello.bolt
```

Using GCC

```shell
cargo run --bin bolt_cli compile --path=./examples/simple.bolt --backend llvm --out="./out/example.ll" --target="x86_64-pc-linux-gnu"
llc -relocation-model=pic ./out/example.ll -o ./out/example.s
as ./out/example.s -o ./out/example.o
gcc ./out/example.o -o ./out/example -lc
./out/example
```

Using Clang

```shell
cargo run --bin bolt_cli compile --path=./examples/simple.bolt --backend llvm --out="./out/example.ll" --target="x86_64-pc-linux-gnu" --bytecode
clang ./out/example.ll -o ./out/example
./out/example
```

Using Shabby JIT

```shell
cargo run --bin bolt_cli jit --path=./examples/simple.bolt --backend llvm
```

# Todo

- [ ] Closures
- [ ] Support for recurssion
- [ ] Loops (for and while)
- [ ] Class / Object syntax
- [ ] Strings
- [ ] STL (Standard Library for different data structures)
- [ ] More binary operators (^ etc)
