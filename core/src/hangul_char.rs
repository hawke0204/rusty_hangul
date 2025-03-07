use crate::choseong::Choseong;
use crate::jongseong::Jongseong;
use crate::jungseong::Jungseong;
use crate::nfd::NFD;
use crate::utils::is_complete_hangul;

#[derive(Debug)]
pub struct HangulChar {
  pub value: char,
  pub unicode: u32,
  pub choseong: Choseong,
  pub jungseong: Jungseong,
  pub jongseong: Option<Jongseong>,
}

impl HangulChar {
  pub fn new(ch: char) -> Option<Self> {
    let unicode = ch as u32;

    if !is_complete_hangul(unicode) {
      return None;
    }

    let NFD(cho, jung, jong) = NFD::normalize(unicode).unwrap();

    Some(Self {
      value: ch,
      unicode,
      choseong: Choseong::new(cho),
      jungseong: Jungseong::new(jung),
      jongseong: jong.map(Jongseong::new),
    })
  }

  pub fn disassemble(&self) -> String {
    let jong = self
      .jongseong
      .as_ref()
      .map_or(String::new(), |j| j.compatibility_value.to_string());

    format!(
      "{}{}{}",
      self.choseong.compatibility_value, self.jungseong.compatibility_value, jong
    )
  }

  pub fn has_batchim(&self) -> bool {
    self.jongseong.is_some()
  }

  pub fn get_choseong(&self) -> char {
    self.choseong.compatibility_value
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_hangul_decomposition() {
    let hangul = HangulChar::new('한').unwrap();

    // 유니코드 검증
    assert_eq!(hangul.unicode, 0xD55C);

    // 초성 검증
    assert_eq!(hangul.choseong.conjoining_value, 'ᄒ');
    assert_eq!(hangul.choseong.compatibility_value, 'ㅎ');

    // 중성 검증
    assert_eq!(hangul.jungseong.conjoining_value, 'ᅡ');
    assert_eq!(hangul.jungseong.compatibility_value, 'ㅏ');

    // 종성 검증
    let jongseong = hangul.jongseong.unwrap();
    assert_eq!(jongseong.conjoining_value, 'ᆫ');
    assert_eq!(jongseong.compatibility_value, 'ㄴ');
  }

  #[test]
  fn test_compatibility_value() {
    let hangul = HangulChar::new('한').unwrap();
    assert_eq!(hangul.disassemble(), "ㅎㅏㄴ");

    let hangul = HangulChar::new('가').unwrap();
    assert_eq!(hangul.disassemble(), "ㄱㅏ");
  }

  #[test]
  fn test_invalid_hangul() {
    assert!(HangulChar::new('a').is_none());
    assert!(HangulChar::new('ㄱ').is_none());
  }

  #[test]
  fn test_has_batchim() {
    let hangul = HangulChar::new('한').unwrap();
    assert!(hangul.has_batchim());

    let hangul = HangulChar::new('하').unwrap();
    assert!(!hangul.has_batchim());
  }

  #[test]
  fn test_get_choseong() {
    let hangul = HangulChar::new('한').unwrap();
    assert_eq!(hangul.get_choseong(), 'ㅎ');

    let hangul = HangulChar::new('가').unwrap();
    assert_eq!(hangul.get_choseong(), 'ㄱ');
  }
}
