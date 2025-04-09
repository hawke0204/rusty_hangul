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
    let test_cases = [
      (0xAC00, (0x1100, 0x1161, None)),
      (0xB098, (0x1102, 0x1161, None)),
      (0xB2E4, (0x1103, 0x1161, None)),
      (0xB77C, (0x1105, 0x1161, None)),
      (0xAC01, (0x1100, 0x1161, Some(0x11A8))),
      (0xB2EC, (0x1103, 0x1161, Some(0x11AF))),
      (0xB9E8, (0x1106, 0x1162, Some(0x11AB))),
      (0xBD93, (0x1107, 0x116E, Some(0x11BA))),
      (0xD55C, (0x1112, 0x1161, Some(0x11AB))),
      (0xD7A3, (0x1112, 0x1175, Some(0x11C2))),
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
  fn test_normalize_invalid_inputs() {
    let invalid_inputs = [
      'a' as u32,
      '1' as u32,
      '@' as u32,
      'ㄱ' as u32,
      'ㅏ' as u32,
      0x1100,
      0x1161,
      0x11A8,
      0x3200,
      0xABFF,
      0xD7A4,
    ];

    for input in invalid_inputs {
      match NFD::normalize(input) {
        Ok(_) => panic!("Expected Err variant for invalid input: U+{:04X}", input),
        Err(e) => assert!(matches!(e, NormalizeError::InvalidHangul)),
      }
    }
  }

  #[test]
  fn test_is_nfd_hangul() {
    let nfd_ga = "\u{1100}\u{1161}";
    let nfd_no = "\u{1102}\u{1169}";
    let nfd_dal = "\u{1103}\u{1161}\u{11AF}";
    let nfd_man = "\u{1106}\u{1162}\u{11AB}";
    let nfd_han = "\u{1112}\u{1161}\u{11AB}";

    assert!(NFD::is_nfd_hangul(nfd_ga));
    assert!(NFD::is_nfd_hangul(nfd_no));
    assert!(NFD::is_nfd_hangul(nfd_dal));
    assert!(NFD::is_nfd_hangul(nfd_man));
    assert!(NFD::is_nfd_hangul(nfd_han));

    assert!(!NFD::is_nfd_hangul("가"));
    assert!(!NFD::is_nfd_hangul("ᄀ"));
    assert!(!NFD::is_nfd_hangul("ᅡ"));
    assert!(!NFD::is_nfd_hangul("ᄀ가"));
    assert!(!NFD::is_nfd_hangul("가ᅡ"));
    assert!(!NFD::is_nfd_hangul("갈ᆫ"));
    assert!(!NFD::is_nfd_hangul("abc"));
    assert!(!NFD::is_nfd_hangul("ㄱㅏ"));
    assert!(!NFD::is_nfd_hangul(""));
  }
}
