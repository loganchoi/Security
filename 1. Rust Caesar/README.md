# pa01 - Crabby Caesar
Instead of stabbing this Caesar, use Brutus force!

![mmmm...](caesar_with_crab.jpg)

or, with no shell to bash, maybe just use a fork...

Implement the Caesar cipher in rust.
With inspiration from the Python3 code in the book,
write an almost identical program in rust.
We will use the exact same symbol set as the book code: 
`ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 !?.`
This assignment should help you learn both Python3 and Rust a bit better,
and cover the basics of cryptography.

## Deliverables
```
./src/main.rs
./sr/crabby_caesar.rs
```

## How your program will be run generally
```sh
cargo build
timeout 15 ./ <sample_input.txt >sample_output.txt
# Also, see the unit_tests/.
```

## How you should test your program
Run this on your local machine:
```sh
bash grade.sh
# or
./grade.sh

# or 
cargo build
gdb ./target/debug/program
# start <sample_input.txt
```
Then, once you are done, you can verify that you got all your code pushed by checking the CI build on git-classes.
You are not done until it's green, and you verify the number on git-classes!

## Notes
You CAN use the book code as guidance, mimicking the algorithm there.

I suggest configuring gdb with this:
https://github.com/cyrus-and/gdb-dashboard
It'll help later in the class anyway.

## Due date
On Canvas.

## Rust
![](https://www.rustacean.net/more-crabby-things/dancing-ferris.gif)

https://rustacean.net/

Why Rust? 
Software security is a many-trillion dollar problem. 
Later in the class, we'll be starting a section on software security, where I will show you how to do software wrong!
Rust is one of the ways you can do software security right.
Thus, we'll be dabbling in both the positive and negative examples.

The Rust Programming Language is a fast and memory-efficient language useful for any project you can think of.
Companies such as Google, Microsoft, Amazon, Discord, Amazon, etc., are using Rust to make their apps faster and more secure:
* https://foundation.rust-lang.org/members/ (many of the big players, except Apple...)
* https://serokell.io/blog/rust-companies
* https://blog.rust-lang.org/2020/08/18/laying-the-foundation-for-rusts-future.html

Rust is a compiled language, so you have to compile your program before you can test it, and you need a slightly different style of debugger.
One of the neat things about Rust's compiler is that it will not compile your code if you have memory leaks or similar issues (yea, it's that smart).
The compiler will point out where you have written bad code and let you know how to fix it.
This makes Rust safer than a language like C++, where you could have memory problems and not even know it.

They have a great learning resource, aptly named ['The Book'](https://doc.rust-lang.org/book/), which will walk you through the basics and how to get started on your first program. 

https://doc.rust-lang.org/book/

The first 5 chapters contains most of the information you will need for this assignment.
Make sure to pay close attention to the Ownership section.
If you think the logic of your code is right but something is off it may be due to how Rust does memory ownership.

The "rust by example" is also good:
https://www.rust-lang.org/learn

As an example of how to structure your code, and some basic Rust programming, I have a demo assignment in Rust, that goes with the autograder:
https://gitlab.com/classroomcode/grade-sh/rust-example (Note: you may find this particularly helpful!)

### Running your Code
All code should fit in the `src/` directory (we have your files already there).
Your main driver and functions will be in `src/main.rs`.
More information about Cargo and how it is used will be in section 1.3 in ['The Book'](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html).
Below are references to the commands you will use the most.

- `cargo run` : Compiles and runs your code, allowing interaction with it for stdio testing.
    - May require a --bin if you have multiple binaries (as this assignment does with it's unit test).
- `cargo run arg1 arg2 arg3` : Compiles and runs your code, passing along any number of arguments you need, for arg-based testing.
- `cargo build` : Compiles your code and puts it in `target/debug/name_specified_in_cargo.toml`

### Hint
Watch out for modulus versus remainder behavior.
