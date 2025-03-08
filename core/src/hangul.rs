use crate::hangul_letter::HangulLetter;

struct CharUnit {
  original: char,
  hangul: Option<HangulLetter>,
}

pub struct Hangul {
  char_units: Vec<CharUnit>,
  original: String,
}

// TODO: NFD 지원
impl Hangul {
  pub fn new(string: &str) -> Self {
    let char_units = string
      .chars()
      .map(|ch| CharUnit {
        original: ch,
        hangul: HangulLetter::parse_from_char(ch),
      })
      .collect();

    Self {
      char_units,
      original: string.to_string(),
    }
  }

  pub fn original(&self) -> &str {
    &self.original
  }

  pub fn len(&self) -> usize {
    self.char_units.len()
  }

  pub fn is_empty(&self) -> bool {
    self.char_units.is_empty()
  }

  pub fn disassemble(&self) -> String {
    self
      .char_units
      .iter()
      .map(|unit| match &unit.hangul {
        Some(hangul) => hangul.disassemble(),
        None => unit.original.to_string(),
      })
      .collect()
  }

  pub fn get_choseong(&self) -> String {
    self
      .char_units
      .iter()
      .map(|unit| match &unit.hangul {
        Some(hangul) => hangul.choseong.compatibility_value.to_string(),
        None => unit.original.to_string(),
      })
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_hangul_sentence_creation() {
    let sentence = Hangul::new("안녕하세요");
    assert_eq!(sentence.len(), 5);
    assert_eq!(sentence.original(), "안녕하세요");
  }

  #[test]
  fn test_mixed_sentence() {
    let sentence = Hangul::new("Hello 안녕!");

    assert_eq!(sentence.len(), 9);
    assert_eq!(sentence.original(), "Hello 안녕!");
  }

  #[test]
  fn test_disassemble() {
    let sentence = Hangul::new("안녕");
    assert_eq!(sentence.disassemble(), "ㅇㅏㄴㄴㅕㅇ");
  }

  #[test]
  fn test_empty_sentence() {
    let sentence = Hangul::new("");
    assert!(sentence.is_empty());
    assert_eq!(sentence.len(), 0);
  }

  #[test]
  fn test_get_choseong() {
    let sentence = Hangul::new("안녕하세요");
    assert_eq!(sentence.get_choseong(), "ㅇㄴㅎㅅㅇ");

    let mixed = Hangul::new("Hello 안녕!");
    assert_eq!(mixed.get_choseong(), "Hello ㅇㄴ!");
  }
}
