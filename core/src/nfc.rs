use std::error::Error;
use std::fmt;

const HANGUL_BASE: u32 = 0xAC00;

const CHOSEONG_COUNT: u32 = 0x13;
const JUNGSEONG_COUNT: u32 = 0x15;
const JONGSEONG_COUNT: u32 = 0x1C;

const CHOSEONG_BASE: u32 = 0x1100;
const JUNGSEONG_BASE: u32 = 0x1161;
const JONGSEONG_BASE: u32 = 0x11A8;

const JUNGSEONG_AND_JONGSEONG_NUMBER_OF_CASES: u32 = JUNGSEONG_COUNT * JONGSEONG_COUNT;

#[derive(Debug)]
pub enum ComposeError {
  InvalidChoseong,
  InvalidJungseong,
  InvalidJongseong,
}

impl fmt::Display for ComposeError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ComposeError::InvalidChoseong => write!(f, "Invalid choseong code"),
      ComposeError::InvalidJungseong => write!(f, "Invalid jungseong code"),
      ComposeError::InvalidJongseong => write!(f, "Invalid jongseong code"),
    }
  }
}

impl Error for ComposeError {}

pub struct Nfc(pub u32);

impl Nfc {
  pub fn compose(
    choseong: u32,
    jungseong: u32,
    jongseong: Option<u32>,
  ) -> Result<Self, ComposeError> {
    // Validate jamo codes
    if !Self::is_valid_choseong(choseong) {
      return Err(ComposeError::InvalidChoseong);
    }
    if !Self::is_valid_jungseong(jungseong) {
      return Err(ComposeError::InvalidJungseong);
    }
    if let Some(jong) = jongseong {
      if !Self::is_valid_jongseong(jong) {
        return Err(ComposeError::InvalidJongseong);
      }
    }

    let cho_index = choseong - CHOSEONG_BASE;
    let jung_index = jungseong - JUNGSEONG_BASE;
    let jong_index = jongseong.map_or(0, |j| j - JONGSEONG_BASE + 1);

    let hangul = HANGUL_BASE
      + (cho_index * JUNGSEONG_AND_JONGSEONG_NUMBER_OF_CASES)
      + (jung_index * JONGSEONG_COUNT)
      + jong_index;

    Ok(Self(hangul))
  }

  fn is_valid_choseong(code: u32) -> bool {
    (CHOSEONG_BASE..CHOSEONG_BASE + CHOSEONG_COUNT).contains(&code)
  }

  fn is_valid_jungseong(code: u32) -> bool {
    (JUNGSEONG_BASE..JUNGSEONG_BASE + JUNGSEONG_COUNT).contains(&code)
  }

  fn is_valid_jongseong(code: u32) -> bool {
    (JONGSEONG_BASE..JONGSEONG_BASE + JONGSEONG_COUNT - 1).contains(&code)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_compose_basic() {
    // Test '가' (no jongseong)
    let result = Nfc::compose(0x1100, 0x1161, None).unwrap();
    assert_eq!(result.0, 0xAC00);

    // Test '한' (with jongseong)
    let result = Nfc::compose(0x1112, 0x1161, Some(0x11AB)).unwrap();
    assert_eq!(result.0, 0xD55C);
  }

  #[test]
  fn test_compose_invalid_inputs() {
    // Invalid choseong
    assert!(matches!(
      Nfc::compose(0x1000, 0x1161, None),
      Err(ComposeError::InvalidChoseong)
    ));

    // Invalid jungseong
    assert!(matches!(
      Nfc::compose(0x1100, 0x1100, None),
      Err(ComposeError::InvalidJungseong)
    ));

    // Invalid jongseong
    assert!(matches!(
      Nfc::compose(0x1100, 0x1161, Some(0x1100)),
      Err(ComposeError::InvalidJongseong)
    ));
  }

  #[test]
  fn test_compose_various_syllables() {
    let test_cases = [
      ((0x1100, 0x1161, None), 0xAC00),         // 가
      ((0x1102, 0x1169, None), 0xB178),         // 노
      ((0x1103, 0x1161, Some(0x11AF)), 0xB2EC), // 달
      ((0x1106, 0x1162, Some(0x11AB)), 0xB9E8), // 맨
      ((0x1107, 0x116E, Some(0x11BA)), 0xBD93), // 붓
      ((0x1112, 0x1161, Some(0x11AB)), 0xD55C), // 한
    ];

    for ((cho, jung, jong), expected) in test_cases {
      let result = Nfc::compose(cho, jung, jong).unwrap();
      assert_eq!(result.0, expected, "Failed for expected U+{:04X}", expected);
    }
  }
}
