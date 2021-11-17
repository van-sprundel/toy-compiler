# Toy Rust Compiler

### Process
A compiler can be broken down into 4 parts
1. tokenize input
2. parse into AST
3. transform AST
4. generate code

### Sources
- parser: https://pest.rs/
- guideline: https://pest.rs/book/examples/calculator.html
- compiler explained: https://softwareengineering.stackexchange.com/questions/165543/how-to-write-a-very-basic-compiler
- about asts: https://nl.wikipedia.org/wiki/AST


### Notes
If statements 
```rust
if (a < b) {
    c = 2;
    return c;
} else {
    c = 3;
}
```
![img](https://norasandler.com/assets/AST.svg)