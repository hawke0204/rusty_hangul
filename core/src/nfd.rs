const HANGUL_BASE: u32 = 0xAC00;

const CHOSEONG_COUNT: u32 = 0x13;
const JUNGSEONG_COUNT: u32 = 0x15;
const JONGSEONG_COUNT: u32 = 0x1C;

const CHOSEONG_BASE: u32 = 0x1100;
const JUNGSEONG_BASE: u32 = 0x1161;
const JONGSEONG_BASE: u32 = 0x11A8;

const JUNGSEONG_AND_JONGSEONG_NUMBER_OF_CASES: u32 = JUNGSEONG_COUNT * JONGSEONG_COUNT;

pub struct Nfd(pub u32, pub u32, pub Option<u32>);

impl Nfd {
  pub fn normalize(letter_unicode: u32) -> Self {
    Self::normalize_from_u32(letter_unicode)
  }

  fn normalize_from_u32(letter_unicode: u32) -> Self {
    let hangul_code = letter_unicode - HANGUL_BASE;

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

    Self(choseong, jungseong, jongseong)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_normalize_from_u32() {
    let test_cases = ['릴'];
    let Nfd(choseong_code, jungseong_code, jongseong_code) = Nfd::normalize(test_cases[0] as u32);

    assert_eq!(choseong_code, 4357);
    assert_eq!(jungseong_code, 4469);
    assert_eq!(jongseong_code, Some(4527));
  }
}
