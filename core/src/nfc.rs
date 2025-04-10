const HANGUL_BASE: u32 = 0xAC00;
const HANGUL_LAST: u32 = 0xD7A3;

pub struct NFC;

impl NFC {
  pub fn is_nfc_hangul(string: &str) -> bool {
    let mut chars = string.chars();

    if chars.clone().count() != 1 {
      return false;
    }

    let unicode = chars.next().unwrap() as u32;
    Self::is_complete_hangul(unicode)
  }

  pub fn is_nfc_hangul_char(ch: char) -> bool {
    let code = ch as u32;
    (HANGUL_BASE..=HANGUL_LAST).contains(&code)
  }

  pub fn is_complete_hangul(unicode: u32) -> bool {
    HANGUL_BASE <= unicode && unicode <= HANGUL_LAST
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_complete_hangul() {
    assert!(NFC::is_complete_hangul(0xAC00));
    assert!(NFC::is_complete_hangul(0xB098));
    assert!(NFC::is_complete_hangul(0xB2E4));
    assert!(NFC::is_complete_hangul(0xD7A3));

    assert!(!NFC::is_complete_hangul(0xABFF));
    assert!(!NFC::is_complete_hangul(0xD7A4));
    assert!(!NFC::is_complete_hangul(0x3131));
    assert!(!NFC::is_complete_hangul(0x1100));
    assert!(!NFC::is_complete_hangul(0x0041));
  }

  #[test]
  fn test_is_nfc_hangul() {
    assert!(NFC::is_nfc_hangul("가"));
    assert!(NFC::is_nfc_hangul("힣"));
    assert!(NFC::is_nfc_hangul("한"));
    assert!(NFC::is_nfc_hangul("글"));
    assert!(NFC::is_nfc_hangul("놀"));

    assert!(!NFC::is_nfc_hangul(""));
    assert!(!NFC::is_nfc_hangul("가나"));
    assert!(!NFC::is_nfc_hangul("a"));
    assert!(!NFC::is_nfc_hangul("ㄱ"));
    assert!(!NFC::is_nfc_hangul("ᄀ"));

    let nfd_ga = "\u{1100}\u{1161}";
    assert!(!NFC::is_nfc_hangul(nfd_ga));
  }
}
