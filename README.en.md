# rusty_hangul

🚧 This library is currently under development. The API may change.

[한국어](./README.md)

## Introduction

rusty_hangul is a Rust library designed for processing Korean (Hangul) strings. It consists of two main components:

1. **core**: A Rust crate that handles core functionality for Hangul processing
2. **node**: A Node.js library that binds core functionality using [napi-rs](https://napi.rs/)

## Rust(Core) Usage Examples

```rust
// Create a Hangul string
let text = Hangul::new("안녕하세요");

// Disassemble Hangul
assert_eq!(text.disassemble(), "ㅇㅏㄴㄴㅕㅇㅎㅏㅅㅔㅇㅛ");

// Extract initial consonants
assert_eq!(text.get_choseong(), "ㅇㄴㅎㅅㅇ");

// Can handle non-Hangul characters too
let mixed = Hangul::new("Hello 안녕!");
assert_eq!(mixed.get_choseong(), "Hello ㅇㄴ!");
```

> Currently, the library can only properly process Hangul strings in NFC format.

## Node.js Usage Examples

```typescript
// Create a Hangul string
const text = new Hangul("안녕하세요");

// Disassemble Hangul
console.log(text.disassemble()); // "ㅇㅏㄴㄴㅕㅇㅎㅏㅅㅔㅇㅛ"

// Extract initial consonants
console.log(text.getChoseong()); // "ㅇㄴㅎㅅㅇ"

// Can handle non-Hangul characters too
const mixed = new Hangul("Hello 안녕!");
console.log(mixed.getChoseong()); // "Hello ㅇㄴ!"
```