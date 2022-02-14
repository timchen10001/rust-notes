
# Mars Calculator

## Introduction

- [Basic Data Types In Rust](https://doc.rust-lang.org/book/ch03-02-data-types.html)

  1. Booleans

  2. Characters

  3. Integers

  4. Floats

- [OwnerShip](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

  1. Each value in Rust is owned by a variable.

  2. When the owner goes out of scope, the value will be de-allocated.

  3. `Single Owner Rule`: There can only be ONE owner at a time.

- [References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

  1. With `&`, the arg of function will borrow the value from its owner.

  2. Without `&`, the arg of function will copy the value from incoming owner and move the value from its owner.

- [The Rust Standart Library](https://doc.rust-lang.org/std/index.html)