use crate::choseong::Choseong;
use crate::jongseong::Jongseong;
use crate::jungseong::Jungseong;
use crate::nfc::NFC;
use crate::nfd::NFD;

pub struct HangulLetter {
  pub value: String,
  pub unicode: Vec<u32>,
  pub choseong: Choseong,
  pub jungseong: Jungseong,
  pub jongseong: Option<Jongseong>,
}

impl HangulLetter {
  pub fn parse(string: &str) -> Option<Self> {
    if NFC::is_nfc_hangul(string) {
      let ch = string.chars().next().unwrap();
      let unicode = vec![ch as u32];
      let NFD(cho, jung, jong) = NFD::normalize(unicode[0]).unwrap();

      return Some(Self {
        value: ch.to_string(),
        unicode,
        choseong: Choseong::new(cho),
        jungseong: Jungseong::new(jung),
        jongseong: jong.map(Jongseong::new),
      });
    }

    if NFD::is_nfd_hangul(string) {
      let chars: Vec<char> = string.chars().collect();
      let choseong = Choseong::new(chars[0] as u32);
      let jungseong = Jungseong::new(chars[1] as u32);
      let jongseong = if chars.len() == 3 {
        Some(Jongseong::new(chars[2] as u32))
      } else {
        None
      };

      return Some(Self {
        value: string.to_string(),
        unicode: chars.iter().map(|c| *c as u32).collect(),
        choseong,
        jungseong,
        jongseong,
      });
    }

    return None;
  }

  pub fn parse_from_char(nfc_char: char) -> Option<Self> {
    let nfc_string = nfc_char.to_string();
    Self::parse(&nfc_string)
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
    let hangul = HangulLetter::parse_from_char('한').unwrap();

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
    let hangul = HangulLetter::parse_from_char('한').unwrap();
    assert_eq!(hangul.disassemble(), "ㅎㅏㄴ");

    let hangul = HangulLetter::parse_from_char('가').unwrap();
    assert_eq!(hangul.disassemble(), "ㄱㅏ");
  }

  #[test]
  fn test_invalid_hangul() {
    assert!(HangulLetter::parse_from_char('a').is_none());
    assert!(HangulLetter::parse_from_char('ㄱ').is_none());
  }

  #[test]
  fn test_has_batchim() {
    let hangul = HangulLetter::parse_from_char('한').unwrap();
    assert!(hangul.has_batchim());

    let hangul = HangulLetter::parse_from_char('하').unwrap();
    assert!(!hangul.has_batchim());
  }

  #[test]
  fn test_parse_nfc_hangul() {
    // 초성+중성만 있는 경우
    let hangul = HangulLetter::parse("가").unwrap();
    assert_eq!(hangul.value, "가");
    assert_eq!(hangul.unicode, vec![0xAC00]);
    assert_eq!(hangul.choseong.compatibility_value, 'ㄱ');
    assert_eq!(hangul.jungseong.compatibility_value, 'ㅏ');
    assert!(hangul.jongseong.is_none());

    // 초성+중성+종성이 있는 경우
    let hangul = HangulLetter::parse("곻").unwrap();
    assert_eq!(hangul.value, "곻");
    assert_eq!(hangul.choseong.compatibility_value, 'ㄱ');
    assert_eq!(hangul.jungseong.compatibility_value, 'ㅗ');
    assert_eq!(hangul.jongseong.unwrap().compatibility_value, 'ㅎ');

    // 쌍자음이 초성인 경우
    let hangul = HangulLetter::parse("쌍").unwrap();
    assert_eq!(hangul.value, "쌍");
    assert_eq!(hangul.choseong.compatibility_value, 'ㅆ');
    assert_eq!(hangul.jungseong.compatibility_value, 'ㅏ');
    assert_eq!(hangul.jongseong.unwrap().compatibility_value, 'ㅇ');

    // 복합 모음인 경우
    let hangul = HangulLetter::parse("귀").unwrap();
    assert_eq!(hangul.value, "귀");
    assert_eq!(hangul.choseong.compatibility_value, 'ㄱ');
    assert_eq!(hangul.jungseong.compatibility_value, 'ㅟ');
    assert!(hangul.jongseong.is_none());
  }

  #[test]
  fn test_parse_nfd_hangul() {
    // 초성+중성만 있는 경우
    let hangul = HangulLetter::parse("가").unwrap();
    assert_eq!(hangul.value, "가");
    assert_eq!(hangul.unicode, vec![0x1100, 0x1161]);
    assert_eq!(hangul.choseong.compatibility_value, 'ㄱ');
    assert_eq!(hangul.jungseong.compatibility_value, 'ㅏ');
    assert!(hangul.jongseong.is_none());

    // 초성+중성+종성이 있는 경우
    let hangul = HangulLetter::parse("한").unwrap();
    assert_eq!(hangul.value, "한");
    assert_eq!(hangul.unicode, vec![0x1112, 0x1161, 0x11AB]);
    assert_eq!(hangul.choseong.compatibility_value, 'ㅎ');
    assert_eq!(hangul.jungseong.compatibility_value, 'ㅏ');
    assert_eq!(hangul.jongseong.unwrap().compatibility_value, 'ㄴ');
  }

  #[test]
  fn test_parse_invalid_input() {
    // 영문자
    assert!(HangulLetter::parse("a").is_none());

    // 한글 자음/모음
    assert!(HangulLetter::parse("ㄱ").is_none());
    assert!(HangulLetter::parse("ㅏ").is_none());

    // 부적절한 조합
    assert!(HangulLetter::parse("ᄀᄀ").is_none()); // 초성+초성
    assert!(HangulLetter::parse("ᅡᅡ").is_none()); // 중성+중성
    assert!(HangulLetter::parse("ᄀᆫ").is_none()); // 초성+종성

    // 빈 문자열
    assert!(HangulLetter::parse("").is_none());

    // 특수문자
    assert!(HangulLetter::parse("!").is_none());

    // 다중 문자
    assert!(HangulLetter::parse("가나").is_none());
  }
}
