const COMPAT_JAMO_BASE: u32 = 0x3131;
const COMPAT_JAMO_LAST: u32 = 0x318E;

// 호환 자모 확인(Compatibility Jamo)
pub fn is_compatibility_jamo(unicode: u32) -> bool {
  COMPAT_JAMO_BASE <= unicode && unicode <= COMPAT_JAMO_LAST
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_compatibility_jamo() {
    assert!(is_compatibility_jamo('ㄱ' as u32)); // U+3131
    assert!(is_compatibility_jamo('ㅏ' as u32)); // U+314F
    assert!(!is_compatibility_jamo('가' as u32)); // U+AC00
  }
}
