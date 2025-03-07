use crate::utils::is_compatibility_jamo;

// 조합형 종성 범위
const JONGSEONG_BASE: u32 = 0x11A8;
const JONGSEONG_LAST: u32 = 0x11C2;

// 호환형 종성 범위
const COMPAT_JONGSEONG_BASE: u32 = 0x3131;
const COMPAT_JONGSEONG_LAST: u32 = 0x314E;

// 호환형 종성 매핑 테이블
const COMPATIBILITY_JONGSEONG_MAPPING: [u32; 27] = [
  0x3131, 0x3132, 0x3133, 0x3134, 0x3135, 0x3136, 0x3137, 0x3139, 0x313A, 0x313B, 0x313C, 0x313D,
  0x313E, 0x313F, 0x3140, 0x3141, 0x3142, 0x3144, 0x3145, 0x3146, 0x3147, 0x3148, 0x314A, 0x314B,
  0x314C, 0x314D, 0x314E,
];

#[derive(Debug)]
pub struct Jongseong {
  pub conjoining_value: char,
  pub conjoining_unicode: u32,
  pub compatibility_value: char,
  pub compatibility_unicode: u32,
}

impl Jongseong {
  pub fn new(unicode: u32) -> Self {
    Self::new_inner(unicode)
  }

  fn new_inner(unicode: u32) -> Self {
    let (conjoining_jamo, compatibility_jamo) = Self::convert_to_jamo(unicode);

    let conjoining_value = unsafe { std::char::from_u32_unchecked(conjoining_jamo) };
    let compatibility_value = unsafe { std::char::from_u32_unchecked(compatibility_jamo) };

    Self {
      conjoining_value,
      conjoining_unicode: conjoining_jamo,
      compatibility_value,
      compatibility_unicode: compatibility_jamo,
    }
  }

  fn convert_to_jamo(unicode: u32) -> (u32, u32) {
    if Self::is_conjoining_jongseong(unicode) {
      let conjoining_jamo = unicode;
      let compatibility_jamo = Self::conjoining_jongseong_to_compatibility(unicode)
        .expect("조합형 자모를 호환형으로 변환하는데 실패했습니다");
      return (conjoining_jamo, compatibility_jamo);
    }

    if Self::is_compatibility_jongseong(unicode) {
      let conjoining_jamo = Self::compatibility_to_conjoining_jongseong(unicode)
        .expect("호환형 자모를 조합형으로 변환하는데 실패했습니다");
      let compatibility_jamo = unicode;
      return (conjoining_jamo, compatibility_jamo);
    }

    panic!("유효한 종성 유니코드가 아닙니다: {}", unicode)
  }

  // 조합형 종성 확인
  pub fn is_conjoining_jongseong(jongseong_code: u32) -> bool {
    JONGSEONG_BASE <= jongseong_code && jongseong_code <= JONGSEONG_LAST
  }

  // 호환형 종성 확인
  pub fn is_compatibility_jongseong(unicode: u32) -> bool {
    COMPAT_JONGSEONG_BASE <= unicode && unicode <= COMPAT_JONGSEONG_LAST
  }

  fn compatibility_to_conjoining_jongseong(compat: u32) -> Option<u32> {
    if !is_compatibility_jamo(compat) || !Self::is_compatibility_jongseong(compat) {
      return None;
    }

    COMPATIBILITY_JONGSEONG_MAPPING
      .iter()
      .position(|&x| x == compat)
      .map(|i| 0x11A8 + i as u32)
  }

  fn conjoining_jongseong_to_compatibility(jongseong_code: u32) -> Option<u32> {
    if !Self::is_conjoining_jongseong(jongseong_code) {
      return None;
    }

    let offset = jongseong_code - 0x11A8;
    COMPATIBILITY_JONGSEONG_MAPPING
      .get(offset as usize)
      .copied()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_jongseong_conversion() {
    let jongseong = Jongseong::new('ᆨ' as u32);
    assert_eq!(jongseong.conjoining_unicode, 0x11A8);
    assert_eq!(jongseong.conjoining_value, 'ᆨ');
    assert_eq!(jongseong.compatibility_unicode, 0x3131);
    assert_eq!(jongseong.compatibility_value, 'ㄱ');
  }

  #[test]
  fn test_jongseong_from_compatibility() {
    let jongseong = Jongseong::new('ㄱ' as u32);
    assert_eq!(jongseong.conjoining_unicode, 0x11A8);
    assert_eq!(jongseong.conjoining_value, 'ᆨ');
    assert_eq!(jongseong.compatibility_unicode, 0x3131);
    assert_eq!(jongseong.compatibility_value, 'ㄱ');
  }
}
