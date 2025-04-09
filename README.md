# rusty_hangul

🚧 현재 개발 중인 라이브러리입니다. API가 변경될 수 있습니다.

[English](./README.en.md)

## 소개 (Introduction)

rusty_hangul은 한글 문자열 처리를 위해 Rust로 작성된 라이브러리입니다. 두 가지 주요 부분으로 구성됩니다:

1. **core**: 한글 처리를 위한 핵심 기능을 담당하는 Rust 크레이트
2. **node**: [napi-rs](https://napi.rs/)를 이용해 코어 기능을 바인딩한 Node.js 라이브러리

## Rust(Core) 사용 예시

```rust
// 한글 문자열 생성
let text = Hangul::new("안녕하세요");

// 한글 분해
assert_eq!(text.disassemble(), "ㅇㅏㄴㄴㅕㅇㅎㅏㅅㅔㅇㅛ");

// 초성 추출
assert_eq!(text.get_choseong(), "ㅇㄴㅎㅅㅇ");

// 한글이 아닌 문자도 처리 가능
let mixed = Hangul::new("Hello 안녕!");
assert_eq!(mixed.get_choseong(), "Hello ㅇㄴ!");
```

> 현재는 NFC로된 한글 문자열만 정상적으로 받을 수 있습니다.

## Node.js 사용 예시

```typescript
// 한글 문자열 생성
const text = new Hangul("안녕하세요");

// 한글 분해
console.log(text.disassemble()); // "ㅇㅏㄴㄴㅕㅇㅎㅏㅅㅔㅇㅛ"

// 초성 추출
console.log(text.getChoseong()); // "ㅇㄴㅎㅅㅇ"

// 한글이 아닌 문자도 처리 가능
const mixed = new Hangul("Hello 안녕!");
console.log(mixed.getChoseong()); // "Hello ㅇㄴ!"
```