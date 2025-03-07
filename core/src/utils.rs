// 완성형 한글 확인(Complete Hangul)
pub fn is_complete_hangul(unicode: u32) -> bool {
  const HANGUL_BASE: u32 = 0xAC00;
  const HANGUL_LAST: u32 = 0xD7A3;
  HANGUL_BASE <= unicode && unicode <= HANGUL_LAST
}

// 호환 자모 확인(Compatibility Jamo)
pub fn is_compatibility_jamo(unicode: u32) -> bool {
  const COMPAT_JAMO_BASE: u32 = 0x3131;
  const COMPAT_JAMO_LAST: u32 = 0x318E;
  COMPAT_JAMO_BASE <= unicode && unicode <= COMPAT_JAMO_LAST
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_complete_hangul() {
    assert!(is_complete_hangul(0xAC00)); // 가
    assert!(is_complete_hangul(0xD7A3)); // 힣
    assert!(!is_complete_hangul(0xABFF)); // 범위 이전
    assert!(!is_complete_hangul(0xD7A4)); // 범위 이후
  }

  #[test]
  fn test_from_string() {
    assert!(is_complete_hangul('가'.into()));
    assert!(is_complete_hangul('힣'.into()));
    assert!(!is_complete_hangul('a'.into()));
  }

  #[test]
  fn test_compatibility_jamo() {
    assert!(is_compatibility_jamo('ㄱ' as u32)); // U+3131
    assert!(is_compatibility_jamo('ㅏ' as u32)); // U+314F
    assert!(!is_compatibility_jamo('가' as u32)); // U+AC00
  }
}
