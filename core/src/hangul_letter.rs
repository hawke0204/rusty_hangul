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
    let jongseong_compatibility_value = self.jongseong.as_ref().map_or(String::new(), |j| {
      if j.is_complex_jongseong() {
        j.decompose_complex_jongseong().iter().collect::<String>()
      } else {
        j.compatibility_value.to_string()
      }
    });

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
  fn test_parse_nfc_hangul() {
    let hangul = HangulLetter::parse("가").unwrap();
    assert_eq!(hangul.value, "가");
    assert_eq!(hangul.unicode, vec![0xAC00]);
    assert_eq!(hangul.choseong.compatibility_value, 'ㄱ');
    assert_eq!(hangul.jungseong.compatibility_value, 'ㅏ');
    assert!(hangul.jongseong.is_none());

    let hangul = HangulLetter::parse("한").unwrap();
    assert_eq!(hangul.value, "한");
    assert_eq!(hangul.unicode, vec![0xD55C]);
    assert_eq!(hangul.choseong.compatibility_value, 'ㅎ');
    assert_eq!(hangul.jungseong.compatibility_value, 'ㅏ');
    assert_eq!(hangul.jongseong.unwrap().compatibility_value, 'ㄴ');

    let hangul = HangulLetter::parse("쌍").unwrap();
    assert_eq!(hangul.value, "쌍");
    assert_eq!(hangul.choseong.compatibility_value, 'ㅆ');
    assert_eq!(hangul.jungseong.compatibility_value, 'ㅏ');
    assert_eq!(hangul.jongseong.unwrap().compatibility_value, 'ㅇ');

    let hangul = HangulLetter::parse("귀").unwrap();
    assert_eq!(hangul.value, "귀");
    assert_eq!(hangul.choseong.compatibility_value, 'ㄱ');
    assert_eq!(hangul.jungseong.compatibility_value, 'ㅟ');
    assert!(hangul.jongseong.is_none());

    let hangul = HangulLetter::parse("값").unwrap();
    assert_eq!(hangul.value, "값");
    assert_eq!(hangul.choseong.compatibility_value, 'ㄱ');
    assert_eq!(hangul.jungseong.compatibility_value, 'ㅏ');
    assert_eq!(hangul.jongseong.unwrap().compatibility_value, 'ㅄ');
  }

  #[test]
  fn test_parse_nfd_hangul() {
    let nfd_ga = "\u{1100}\u{1161}";
    let hangul = HangulLetter::parse(nfd_ga).unwrap();
    assert_eq!(hangul.value, nfd_ga);
    assert_eq!(hangul.unicode, vec![0x1100, 0x1161]);
    assert_eq!(hangul.choseong.compatibility_value, 'ㄱ');
    assert_eq!(hangul.jungseong.compatibility_value, 'ㅏ');
    assert!(hangul.jongseong.is_none());

    let nfd_han = "\u{1112}\u{1161}\u{11AB}";
    let hangul = HangulLetter::parse(nfd_han).unwrap();
    assert_eq!(hangul.value, nfd_han);
    assert_eq!(hangul.unicode, vec![0x1112, 0x1161, 0x11AB]);
    assert_eq!(hangul.choseong.compatibility_value, 'ㅎ');
    assert_eq!(hangul.jungseong.compatibility_value, 'ㅏ');
    assert_eq!(hangul.jongseong.unwrap().compatibility_value, 'ㄴ');
  }

  #[test]
  fn test_parse_from_char() {
    let hangul = HangulLetter::parse_from_char('한').unwrap();
    assert_eq!(hangul.value, "한");
    assert_eq!(hangul.unicode, vec![0xD55C]);

    assert!(HangulLetter::parse_from_char('a').is_none());
    assert!(HangulLetter::parse_from_char('ㄱ').is_none());
    assert!(HangulLetter::parse_from_char('!').is_none());
  }

  #[test]
  fn test_disassemble() {
    let hangul = HangulLetter::parse("가").unwrap();
    assert_eq!(hangul.disassemble(), "ㄱㅏ");

    let hangul = HangulLetter::parse("한").unwrap();
    assert_eq!(hangul.disassemble(), "ㅎㅏㄴ");

    let hangul = HangulLetter::parse("의").unwrap();
    assert_eq!(hangul.disassemble(), "ㅇㅢ");

    let hangul = HangulLetter::parse("값").unwrap();
    assert_eq!(hangul.disassemble(), "ㄱㅏㅂㅅ");
  }

  #[test]
  fn test_has_batchim() {
    assert!(HangulLetter::parse("한").unwrap().has_batchim());
    assert!(HangulLetter::parse("값").unwrap().has_batchim());

    assert!(!HangulLetter::parse("가").unwrap().has_batchim());
    assert!(!HangulLetter::parse("뉘").unwrap().has_batchim());
  }

  #[test]
  fn test_invalid_input() {
    assert!(HangulLetter::parse("a").is_none());

    assert!(HangulLetter::parse("ㄱ").is_none());
    assert!(HangulLetter::parse("ㅏ").is_none());

    assert!(HangulLetter::parse("ᄀᄀ").is_none());
    assert!(HangulLetter::parse("ᅡᅡ").is_none());
    assert!(HangulLetter::parse("ᄀᆫ").is_none());

    assert!(HangulLetter::parse("").is_none());

    assert!(HangulLetter::parse("!").is_none());

    assert!(HangulLetter::parse("가나").is_none());
  }
}
