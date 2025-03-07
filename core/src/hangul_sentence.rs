use crate::hangul_char::HangulChar;

#[derive(Debug)]
struct CharUnit {
  original: char,
  hangul: Option<HangulChar>,
}

#[derive(Debug)]
pub struct HangulSentence {
  char_units: Vec<CharUnit>,
  original: String,
}

impl HangulSentence {
  pub fn new(text: &str) -> Self {
    let char_units = text
      .chars()
      .map(|ch| CharUnit {
        original: ch,
        hangul: HangulChar::new(ch),
      })
      .collect();

    Self {
      char_units,
      original: text.to_string(),
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
        Some(hangul) => hangul.get_choseong().to_string(),
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
    let sentence = HangulSentence::new("안녕하세요");
    assert_eq!(sentence.len(), 5);
    assert_eq!(sentence.original(), "안녕하세요");
  }

  #[test]
  fn test_mixed_sentence() {
    let sentence = HangulSentence::new("Hello 안녕!");

    println!("{:?}", sentence);

    assert_eq!(sentence.len(), 9);
    assert_eq!(sentence.original(), "Hello 안녕!");
  }

  #[test]
  fn test_disassemble() {
    let sentence = HangulSentence::new("안녕");
    assert_eq!(sentence.disassemble(), "ㅇㅏㄴㄴㅕㅇ");
  }

  #[test]
  fn test_empty_sentence() {
    let sentence = HangulSentence::new("");
    assert!(sentence.is_empty());
    assert_eq!(sentence.len(), 0);
  }

  #[test]
  fn test_get_choseong() {
    let sentence = HangulSentence::new("안녕하세요");
    assert_eq!(sentence.get_choseong(), "ㅇㄴㅎㅅㅇ");

    let mixed = HangulSentence::new("Hello 안녕!");
    assert_eq!(mixed.get_choseong(), "Hello ㅇㄴ!");
  }
}
