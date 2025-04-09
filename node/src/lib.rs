#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn hangul_disassemble(text: String) -> String {
  let text = hangul::Hangul::new(&text);
  text.disassemble()
}

#[napi]
pub fn hangul_get_choseong(text: String) -> String {
  let text = hangul::Hangul::new(&text);
  text.get_choseong()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_hangul_disassemble_basic() {
    assert_eq!(hangul_disassemble("안녕".to_string()), "ㅇㅏㄴㄴㅕㅇ");
    assert_eq!(hangul_disassemble("가나다".to_string()), "ㄱㅏㄴㅏㄷㅏ");
    assert_eq!(hangul_disassemble("한글".to_string()), "ㅎㅏㄴㄱㅡㄹ");
  }

  #[test]
  fn test_hangul_disassemble_with_non_hangul() {
    assert_eq!(
      hangul_disassemble("Hello 안녕!".to_string()),
      "Hello ㅇㅏㄴㄴㅕㅇ!"
    );
    assert_eq!(
      hangul_disassemble("123 한글 ABC".to_string()),
      "123 ㅎㅏㄴㄱㅡㄹ ABC"
    );
  }

  #[test]
  fn test_hangul_disassemble_empty_string() {
    assert_eq!(hangul_disassemble("".to_string()), "");
  }

  #[test]
  fn test_hangul_disassemble_complex_syllables() {
    assert_eq!(hangul_disassemble("꿈".to_string()), "ㄲㅜㅁ");
    assert_eq!(hangul_disassemble("밝다".to_string()), "ㅂㅏㄹㄱㄷㅏ");
    assert_eq!(hangul_disassemble("닭고기".to_string()), "ㄷㅏㄹㄱㄱㅗㄱㅣ");
  }

  #[test]
  fn test_hangul_disassemble_with_spaces() {
    assert_eq!(
      hangul_disassemble("안녕 하세요".to_string()),
      "ㅇㅏㄴㄴㅕㅇ ㅎㅏㅅㅔㅇㅛ"
    );
  }

  #[test]
  fn test_hangul_get_choseong_basic() {
    assert_eq!(hangul_get_choseong("안녕".to_string()), "ㅇㄴ");
    assert_eq!(hangul_get_choseong("가나다".to_string()), "ㄱㄴㄷ");
    assert_eq!(hangul_get_choseong("한글".to_string()), "ㅎㄱ");
  }

  #[test]
  fn test_hangul_get_choseong_with_non_hangul() {
    assert_eq!(
      hangul_get_choseong("Hello 안녕!".to_string()),
      "Hello ㅇㄴ!"
    );
    assert_eq!(
      hangul_get_choseong("123 한글 ABC".to_string()),
      "123 ㅎㄱ ABC"
    );
  }

  #[test]
  fn test_hangul_get_choseong_empty_string() {
    assert_eq!(hangul_get_choseong("".to_string()), "");
  }

  #[test]
  fn test_hangul_get_choseong_complex_syllables() {
    assert_eq!(hangul_get_choseong("꿈".to_string()), "ㄲ");
    assert_eq!(hangul_get_choseong("밝다".to_string()), "ㅂㄷ");
    assert_eq!(hangul_get_choseong("닭고기".to_string()), "ㄷㄱㄱ");
  }

  #[test]
  fn test_hangul_get_choseong_with_spaces() {
    assert_eq!(
      hangul_get_choseong("안녕 하세요".to_string()),
      "ㅇㄴ ㅎㅅㅇ"
    );
  }
}
