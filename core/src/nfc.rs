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

  pub fn is_complete_hangul(unicode: u32) -> bool {
    HANGUL_BASE <= unicode && unicode <= HANGUL_LAST
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_nfc_hangul() {
    // 유효한 NFC 한글
    assert!(NFC::is_nfc_hangul("가"));
    assert!(NFC::is_nfc_hangul("힣"));
    assert!(NFC::is_nfc_hangul("한"));

    // 유효하지 않은 NFC 한글
    assert!(!NFC::is_nfc_hangul("ㄱㅏ"));
    assert!(!NFC::is_nfc_hangul("ㄱ"));
    assert!(!NFC::is_nfc_hangul("가나"));
    assert!(!NFC::is_nfc_hangul("a"));
    assert!(!NFC::is_nfc_hangul(""));
  }

  #[test]
  fn test_complete_hangul() {
    assert!(NFC::is_complete_hangul(0xAC00)); // 가
    assert!(NFC::is_complete_hangul(0xD7A3)); // 힣
    assert!(!NFC::is_complete_hangul(0xABFF)); // 범위 이전
    assert!(!NFC::is_complete_hangul(0xD7A4)); // 범위 이후
  }
}
