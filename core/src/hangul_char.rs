use crate::choseong::Choseong;
use crate::jongseong::Jongseong;
use crate::jungseong::Jungseong;
use crate::nfd::NFD;
use crate::utils::is_nfc_hangul;

pub struct HangulChar {
  pub value: char,
  pub unicode: Vec<u32>, // NFC는 1개의 유니코드이지만, NFD는 2개 이상의 유니코드를 가질 수 있다.
  pub choseong: Choseong,
  pub jungseong: Jungseong,
  pub jongseong: Option<Jongseong>,
}

// TODO: NFD를 인자로 받아서 파싱하는 기능 추가
impl HangulChar {
  pub fn parse(string: &str) -> Option<Self> {
    if !is_nfc_hangul(string) {
      return None;
    }

    let ch = string.chars().next().unwrap();
    let unicode = vec![ch as u32];
    let NFD(cho, jung, jong) = NFD::normalize(unicode[0]).unwrap();

    Some(Self {
      value: ch,
      unicode,
      choseong: Choseong::new(cho),
      jungseong: Jungseong::new(jung),
      jongseong: jong.map(Jongseong::new),
    })
  }

  pub fn parse_from_char(ch: char) -> Option<Self> {
    let string = ch.to_string();

    if !is_nfc_hangul(&string) {
      return None;
    }

    Self::parse(&string)
  }

  pub fn disassemble(&self) -> String {
    let jongseong_compatibility_value = self
      .jongseong
      .as_ref()
      .map_or(String::new(), |j| j.compatibility_value.to_string());

    format!(
      "{}{}{}",
      self.choseong.compatibility_value,
      self.jungseong.compatibility_value,
      jongseong_compatibility_value
    )
  }

  pub fn has_batchim(&self) -> bool {
    self.jongseong.is_some()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_hangul_decomposition() {
    let hangul = HangulChar::parse_from_char('한').unwrap();

    // 유니코드 검증
    assert_eq!(hangul.unicode[0], 0xD55C);

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
    let hangul = HangulChar::parse_from_char('한').unwrap();
    assert_eq!(hangul.disassemble(), "ㅎㅏㄴ");

    let hangul = HangulChar::parse_from_char('가').unwrap();
    assert_eq!(hangul.disassemble(), "ㄱㅏ");
  }

  #[test]
  fn test_invalid_hangul() {
    assert!(HangulChar::parse_from_char('a').is_none());
    assert!(HangulChar::parse_from_char('ㄱ').is_none());
  }

  #[test]
  fn test_has_batchim() {
    let hangul = HangulChar::parse_from_char('한').unwrap();
    assert!(hangul.has_batchim());

    let hangul = HangulChar::parse_from_char('하').unwrap();
    assert!(!hangul.has_batchim());
  }
}
