# rusty_hangul

🚧 현재 개발 중인 라이브러리입니다. API가 변경될 수 있습니다.

## 현재 지원하는 기능

### 한글 문자열 처리
```rust
use rusty_hangul::Hangul;

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

## 개발 예정 기능
- 중성, 종성 검색
- 문자열에 적합한 조사 추가 (예: 을/를, 이/가)
- 받침 존재 여부 확인
- 한글 로마자 변환
- 한글 표준 발음법 적용

