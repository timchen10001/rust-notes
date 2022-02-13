# Rust Notes

## About Rust

*`"Rust is blazing fast and memory-efficient: with no run-time and garbage collector."`*

- No Run Time.
- No Garbage Collector.

*`"Rust's rich type system and ownership model gurantee memory-safety and thread-safety"`*

- Type system and ownership model.
- Safety on memory and thread.

*`Rust is a modern system programming language.`*

- Memory Safe.
- No Null. (`No more referencing, No pointers exceptions`)
- No Exception. 
- Modern Package Management (`Cargo`)

## Sources

- ## [Rust](https://doc.rust-lang.org/book/title-page.html)
- ## [Crates](https://crates.io/)

## Memory Management (C/C++ Reviewing)

- Stack
- Heap
- Pointers
- Smart Points

## What is the `Stack` ?

- It's special region of the process memory that stores variables created by each function.

- For every function call a new stack frame is allocated on top of the current one.

- `The Size of every variable on the stack has to be known at complie time.`
  > If we want to store an array on the stack, we have to specify exactly how many elements it will hold.

- `When a function exits it's stack frame is released.`
  > This means that we don't have to dislocating he memory.

- *`It's important to remeber that the stack has a limited size determined by the machine architecture ... etc factors`*
  > If we reach the end of the stack by an infinite recursion, for example, our program will crash with a `stack over flow` error.

## What is the `Heap` ?
  > and how it is different from the `Stack`

- It's a region of the process memory that is `NOT` automatically managed.

- It has no size restrictions.
  
- It's `accessible by any function, anywhere in program`.

- `Heap allocations are expensive and we should void them when possible`.
  > Because if a program allocates a lot of bloacks on the heap, eventually the heap will be fragmented.

  > This makes it much harder to efficiently find the neccessary space for new allocations.

## Discussions
1. It's too hard to safely manage memory on language like `C` or `C++`.
2. Memory Safety is not an issue on higher level language like `Javascript`, `Java` or `Go` where the language comes with a `heavy runtime` and a `garbage collector` that take care of the unused memory.
3. But the `heavy runtime` is unfortunately not an option for system languages.

## Modern Solution - Smart Pointers
- `A pointer wrapper that gurantees that once the pointer is destroied, it will make sure to go ahead and dislocate the memory on heap region.`

- `Box` is a type of smart pointer in rust.