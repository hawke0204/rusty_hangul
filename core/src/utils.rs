pub fn is_complete_hangul_from_u32(letter_unicode: u32) -> bool {
  const HANGUL_BASE: u32 = 0xAC00;
  const HANGUL_LAST: u32 = 0xD7A3;
  HANGUL_BASE <= letter_unicode && letter_unicode <= HANGUL_LAST
}

pub fn is_choseong_from_u32(choseong_code: u32) -> bool {
  const CHOSEONG_BASE: u32 = 0x1100;
  const CHOSEONG_LAST: u32 = 0x1112;
  CHOSEONG_BASE <= choseong_code && choseong_code <= CHOSEONG_LAST
}

pub fn is_jungseong_from_u32(jungseong_code: u32) -> bool {
  const JUNGSEONG_BASE: u32 = 0x1161;
  const JUNGSEONG_LAST: u32 = 0x1175;
  JUNGSEONG_BASE <= jungseong_code && jungseong_code <= JUNGSEONG_LAST
}

pub fn is_jongseong_from_u32(jongseong_code: u32) -> bool {
  const JONGSEONG_BASE: u32 = 0x11A8;
  const JONGSEONG_LAST: u32 = 0x11C2;
  JONGSEONG_BASE <= jongseong_code && jongseong_code <= JONGSEONG_LAST
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_complete_hangul() {
    assert!(is_complete_hangul_from_u32(0xAC00)); // 가
    assert!(is_complete_hangul_from_u32(0xD7A3)); // 힣
    assert!(!is_complete_hangul_from_u32(0xABFF)); // 범위 이전
    assert!(!is_complete_hangul_from_u32(0xD7A4)); // 범위 이후
  }

  #[test]
  fn test_jamo() {
    assert!(is_choseong_from_u32(0x1100)); // ㄱ
    assert!(is_jungseong_from_u32(0x1161)); // ㅏ
    assert!(is_jongseong_from_u32(0x11A8)); // ㄱ

    assert!(!is_choseong_from_u32(0x1113)); // 범위 초과
    assert!(!is_jungseong_from_u32(0x1176)); // 범위 초과
    assert!(!is_jongseong_from_u32(0x11C3)); // 범위 초과
  }

  #[test]
  fn test_from_string() {
    assert!(is_complete_hangul_from_u32('가'.into()));
    assert!(is_complete_hangul_from_u32('힣'.into()));
    assert!(!is_complete_hangul_from_u32('a'.into()));

    assert!(is_choseong_from_u32('ᄀ'.into())); // 초성 ㄱ
    assert!(is_choseong_from_u32('ᄒ'.into())); // 초성 ㅎ

    assert!(is_jungseong_from_u32('ᅡ'.into())); // 중성 ㅏ
    assert!(is_jungseong_from_u32('ᅵ'.into())); // 중성 ㅣ

    assert!(is_jongseong_from_u32('ᆨ'.into())); // 종성 ㄱ
    assert!(is_jongseong_from_u32('ᆺ'.into())); // 종성 ㅅ

    assert!(!is_complete_hangul_from_u32('A'.into()));
    assert!(!is_choseong_from_u32('가'.into()));
    assert!(!is_jungseong_from_u32('ᄀ'.into()));
    assert!(!is_jongseong_from_u32('ᅡ'.into()));
  }
}
