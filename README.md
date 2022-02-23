# rusty_beginnings
First steps into the RUST programming language
## Alternatively you can 
Create a .rs file
run the command from cmd : rustc <filename>.rs 
it creates a .exe file which can be run by just typing the exe filename in cmd
## From folder structure
cargo init
## cd to src folder
cargo run
cargo build
## Build for production release
cargo build --release

## Variables
By default RUST variable are immutable to always enable ; safety, concurrency and speed.
## Variable Binding : 
Declaring a variable without initialization. Scope is Local.
When an object goes out of scope, its destructor is called and memory freed. hence rust is memory safe and you dont have to worry about memory leaks.

## Data Types
Rust is statically typed; it must know the types of all variables at compile time. Data types in rust include integer, floating point, boolean and character.
Integer can be signed or unsigned(default) and has a range value deepending on size of the bit used from 8 to 128.[-(2^n-1) to (2^n-1)-1]
## Compound data types :
Data types which allows for the grouping of multiple/different data types. Rust provides two primitve data types, tuples and arrays. Tuples have a fixed length, hence once declared cannot grow in size. Arrays are comma seperated vales square braces, has a fixed length with same datatype. 

## Data Structures
if else
if else if
loop
while
for 