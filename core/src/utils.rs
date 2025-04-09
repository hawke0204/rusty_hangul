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
    assert!(is_compatibility_jamo('ㄱ' as u32));
    assert!(is_compatibility_jamo('ㄴ' as u32));
    assert!(is_compatibility_jamo('ㅏ' as u32));
    assert!(is_compatibility_jamo('ㅣ' as u32));
    assert!(is_compatibility_jamo('ㅥ' as u32));
    assert!(is_compatibility_jamo('ㆎ' as u32));

    assert!(!is_compatibility_jamo('가' as u32));
    assert!(!is_compatibility_jamo('a' as u32));
    assert!(!is_compatibility_jamo('1' as u32));
    assert!(!is_compatibility_jamo('ᄀ' as u32));
  }
}
