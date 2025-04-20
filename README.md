Installation:
download the exe file from website and run it

- rustc is the rust compiler which creates executable for running rust program similar to node for  nodejs
- rustup is cli for updating the version of the rust similar to nvm for nodejs
- rustfmt is cli command to format the rust code similiar to prittier
- Rustaceans is the name for rust developer
- cargo is the package manager like npm
- Packages are know as crates in rust

## Cargo
Create project with cargo
```shell
cargo new <Project_name>
```

- packages is known as crates
- run program in cargo by command:
  ```shell
  cargo run
  ```
  Note: it will build and run

- for release(production) the code use command:
  ```shell
    cargo build --release
  ```

- for checking the code use command:
  ```shell
    cargo check
  ```
    Note: This will speed up development process by providing quicker way to check if updated code still compiles. This does not generates any executable.

- To install any crate write the crate name with version in Cargo.toml file.  <br>Example:
  rand = "0.9.0"
**Alternative using command**
```shell
  cargo add <package_name>
```

- To remove crates use command:
```shell
  cargo remove <package-name>
```

- To update the crates use command:
  ```shell
    cargo update
  ```

- To open documentation of all the crates and project use command:
  ```shell
    cargo doc --open
  ```

## Variables & Mutability
Variables are created by using keyword "let".
- A variable is immutable by default mean the variable cannot be re-assigned. To re-assign the "mut" keyword shouild be used.

  ```shell
    let mut age = 5;
  ```

- "const" keyword is used to create a Constant which is also immutable(Once assigned it's value can't be changed).
  - It never be re-assigned.
  - It is Required to define type of the constant(must be annotated).
  - It can have Global or Local scope. 
  - Cannot assign let variable as value of const variable
    
  ```shell
    let age = 5;
    const myfutureAge: u32 = 20 + age; //it will throw an error.
  ```
  - Use Capital letters to name a constant.  

Shadowing: is the way to re-declare the variable.<br>

  ```shell
    let mut age = 10;
    age = age + 1;
    let age = age;
  ```

The advantage of using shadowing is:
  - After mutating the value, we can make it immutable.
  - The variable type can be changed like string to number.(Making use of only one variable instead seperate variable for string and number).
    
## Data types
Unlike JavaScript which is loosely written dynamic language, Rust is a statically typed language which means it must needs to know data type of a variable at compile time. 
Although the compiler can infer the type of variable, in cases where you need a specific type of variable you must explicitly annotate it.

- Integer type:
  - Can be unsigned integer or signed intger. 
  - Can have 8, 16, 32, 64 and 128 bits form available.
  - If the bit architecture of a computer is not known then a 'arch' can be used.

- Floating type:
  - Floats are always signed.
  - Can have 32 and 64 bit size only.
  - The 64 bits floats have higher precision (numbers after decimal) than 32 bit float.

- Character type:
  - Four bytes in size.
  - It is a Unicode scalar value and can store emojis and many more things.

- Tuple type:
  - Used to group together a number of values with different data types.
  - Have fixed length, Once decalred can't grow or shrink.
  - An tuple with no elements or an empty tuple is called as a "Unit".

- Array type: 
  - Used for grouping number of values having same data type.
  - Fixed length.

## Functions
"fn" keyword is used to define a function.
type annotation is necesary in case of parameterized functions to know type of parameters.

- Statements vs Expressions
  - A function is made up of series of statements.
  - Statements are instructions that perform some action and do not return a value.
  - Expression has a resultant value or returns something.
  - 5 + 6, which is an expression that evaluates to the value 11. 
  - So function declaration is a statement while function calling is an expression.
  - Expressions do not include ending semicolons. 
  - If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value

- Functions with return values
  - The type of the return value must be declared after an arrow (->)
  - You can return early from a function by using the return keyword and specifying a value
  - Most functions return the last expression implicitly. 
