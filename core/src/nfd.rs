use std::error::Error;
use std::fmt;

use crate::choseong::Choseong;
use crate::jongseong::Jongseong;
use crate::jungseong::Jungseong;
use crate::nfc::NFC;

const HANGUL_BASE: u32 = 0xAC00;

// const CHOSEONG_COUNT: u32 = 0x13;
const JUNGSEONG_COUNT: u32 = 0x15;
const JONGSEONG_COUNT: u32 = 0x1C;

const CHOSEONG_BASE: u32 = 0x1100;
const JUNGSEONG_BASE: u32 = 0x1161;
const JONGSEONG_BASE: u32 = 0x11A8;

const JUNGSEONG_AND_JONGSEONG_NUMBER_OF_CASES: u32 = JUNGSEONG_COUNT * JONGSEONG_COUNT;

pub struct NFD(pub u32, pub u32, pub Option<u32>);

#[derive(Debug)]
pub enum NormalizeError {
  InvalidHangul,
}

impl fmt::Display for NormalizeError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      NormalizeError::InvalidHangul => write!(f, "Not a valid hangul character"),
    }
  }
}

impl Error for NormalizeError {}

impl NFD {
  pub fn normalize(nfc_letter_unicode: u32) -> Result<Self, NormalizeError> {
    if !NFC::is_complete_hangul(nfc_letter_unicode) {
      return Err(NormalizeError::InvalidHangul);
    }

    let hangul_code = nfc_letter_unicode - HANGUL_BASE;

    let choseong_index = hangul_code / (JUNGSEONG_AND_JONGSEONG_NUMBER_OF_CASES);
    let jungseong_index = (hangul_code % JUNGSEONG_AND_JONGSEONG_NUMBER_OF_CASES) / JONGSEONG_COUNT;
    let jongseong_index = hangul_code % JONGSEONG_COUNT;

    let choseong = CHOSEONG_BASE + choseong_index;
    let jungseong = JUNGSEONG_BASE + jungseong_index;
    let jongseong = if jongseong_index > 0 {
      Some(JONGSEONG_BASE + jongseong_index - 1)
    } else {
      None
    };

    Ok(Self(choseong, jungseong, jongseong))
  }

  pub fn is_nfd_hangul(string: &str) -> bool {
    let chars: Vec<char> = string.chars().collect();
    let chars_len = chars.len();

    if chars_len != 2 && chars_len != 3 {
      return false;
    }

    let choseong_unicode = chars[0] as u32;
    let jungseong_unicode = chars[1] as u32;

    if !Choseong::is_conjoining_choseong(choseong_unicode) {
      return false;
    }

    if !Jungseong::is_conjoining_jungseong(jungseong_unicode) {
      return false;
    }

    if chars_len == 3 {
      let jongseong_unicode = chars[2] as u32;
      if !Jongseong::is_conjoining_jongseong(jongseong_unicode) {
        return false;
      }
    }

    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_normalize_from_u32() {
    let result = NFD::normalize('릴' as u32);
    match result {
      Ok(NFD(choseong_code, jungseong_code, jongseong_code)) => {
        assert_eq!(choseong_code, 4357);
        assert_eq!(jungseong_code, 4469);
        assert_eq!(jongseong_code, Some(4527));
      }
      Err(_) => panic!("Expected Ok variant for valid hangul character"),
    }
  }

  #[test]
  fn test_normalize_invalid_inputs() {
    let invalid_inputs = ['a', '1', '@', 'Z', 'ㄱ', 'ㅏ', 'ㄴ', '\u{3200}'];

    for input in invalid_inputs {
      match NFD::normalize(input as u32) {
        Ok(_) => panic!(
          "Expected Err variant for invalid hangul character: {}",
          input
        ),
        Err(e) => assert!(matches!(e, NormalizeError::InvalidHangul)),
      }
    }
  }

  #[test]
  fn test_normalize_various_syllables() {
    let test_cases = [
      (0xAC00, (0x1100, 0x1161, None)),         // 가
      (0xB178, (0x1102, 0x1169, None)),         // 노
      (0xB2EC, (0x1103, 0x1161, Some(0x11AF))), // 달
      (0xB9E8, (0x1106, 0x1162, Some(0x11AB))), // 맨
      (0xBD93, (0x1107, 0x116E, Some(0x11BA))), // 붓
      (0xD55C, (0x1112, 0x1161, Some(0x11AB))), // 한
    ];

    for (input, expected) in test_cases {
      match NFD::normalize(input) {
        Ok(NFD(cho, jung, jong)) => {
          assert_eq!(
            (cho, jung, jong),
            expected,
            "Failed for character U+{:04X}",
            input
          );
        }
        Err(e) => panic!("Normalization failed for U+{:04X} with error: {}", input, e),
      }
    }
  }

  #[test]
  fn test_is_nfd_hangul() {
    // 유효한 NFD 문자열
    assert!(NFD::is_nfd_hangul("가")); // 가
    assert!(NFD::is_nfd_hangul("넌")); // 넌
    assert!(NFD::is_nfd_hangul("한")); // 한

    // 유효하지 않은 NFD 문자열
    assert!(!NFD::is_nfd_hangul("가")); // NFC
    assert!(!NFD::is_nfd_hangul("ᄀ")); // 초성만
    assert!(!NFD::is_nfd_hangul("간ᄀ")); // 4자
    assert!(!NFD::is_nfd_hangul("abc")); // 영문
    assert!(!NFD::is_nfd_hangul("ㄱㅏ")); // 호환형 자모
  }
}
