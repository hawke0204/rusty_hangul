use crate::utils::is_compatibility_jamo;

// 조합형 초성 범위
const CHOSEONG_BASE: u32 = 0x1100;
const CHOSEONG_LAST: u32 = 0x1112;

// 호환형 초성 범위
const COMPAT_CHOSEONG_BASE: u32 = 0x3131;
const COMPAT_CHOSEONG_LAST: u32 = 0x314E;

// 호환형 초성 매핑 테이블
const COMPATIBILITY_CHOSEONG_MAPPING: [u32; 19] = [
  0x3131, 0x3132, 0x3134, 0x3137, 0x3138, 0x3139, 0x3141, 0x3142, 0x3143, 0x3145, 0x3146, 0x3147,
  0x3148, 0x3149, 0x314A, 0x314B, 0x314C, 0x314D, 0x314E,
];

#[derive(Debug)]
pub struct Choseong {
  pub conjoining_value: char,
  pub conjoining_unicode: u32,
  pub compatibility_value: char,
  pub compatibility_unicode: u32,
}

impl Choseong {
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
    if Self::is_conjoining_choseong(unicode) {
      let compatibility_jamo = Self::conjoining_choseong_to_compatibility(unicode)
        .expect("조합형 자모를 호환형으로 변환하는데 실패했습니다");
      return (unicode, compatibility_jamo);
    }

    if Self::is_compatibility_choseong(unicode) {
      let conjoining_jamo = Self::compatibility_to_conjoining_choseong(unicode)
        .expect("호환형 자모를 조합형으로 변환하는데 실패했습니다");
      return (conjoining_jamo, unicode);
    }

    panic!("유효한 초성 유니코드가 아닙니다: {}", unicode);
  }

  // 조합형 초성 확인(Conjoining Choseong)
  pub fn is_conjoining_choseong(choseong_code: u32) -> bool {
    CHOSEONG_BASE <= choseong_code && choseong_code <= CHOSEONG_LAST
  }

  // 호환형 초성 확인(Compatibility Choseong)
  pub fn is_compatibility_choseong(unicode: u32) -> bool {
    COMPAT_CHOSEONG_BASE <= unicode && unicode <= COMPAT_CHOSEONG_LAST
  }

  fn conjoining_choseong_to_compatibility(choseong_code: u32) -> Option<u32> {
    if !Self::is_conjoining_choseong(choseong_code) {
      return None;
    }

    let offset = choseong_code - 0x1100;
    COMPATIBILITY_CHOSEONG_MAPPING.get(offset as usize).copied()
  }

  fn compatibility_to_conjoining_choseong(compat: u32) -> Option<u32> {
    if !Self::is_compatibility_choseong(compat) {
      return None;
    }

    COMPATIBILITY_CHOSEONG_MAPPING
      .iter()
      .position(|&x| x == compat)
      .map(|i| 0x1100 + i as u32)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_choseong() {
    let choseong = Choseong::new('ㄱ' as u32);

    assert_eq!(choseong.conjoining_unicode, 0x1100);
    assert_eq!(choseong.conjoining_value, 'ᄀ');
    assert_eq!(choseong.compatibility_unicode, 0x3131);
    assert_eq!(choseong.compatibility_value, 'ㄱ');
  }

  #[test]
  fn test_choseong_from_compatibility() {
    let choseong = Choseong::new('ㄱ' as u32);

    assert_eq!(choseong.conjoining_unicode, 0x1100);
    assert_eq!(choseong.conjoining_value, 'ᄀ');
    assert_eq!(choseong.compatibility_unicode, 0x3131);
    assert_eq!(choseong.compatibility_value, 'ㄱ');
  }

  #[test]
  fn test_choseong_conversion() {
    assert!(Choseong::is_conjoining_choseong(0x1100)); // ㄱ
    assert!(!Choseong::is_conjoining_choseong(0x1113)); // 범위 초과

    assert_eq!(
      Choseong::conjoining_choseong_to_compatibility('ᄀ' as u32),
      Some('ㄱ' as u32)
    );
    assert_eq!(
      Choseong::compatibility_to_conjoining_choseong('ㄱ' as u32),
      Some('ᄀ' as u32)
    );
  }

  #[test]
  fn test_compatibility_choseong() {
    assert!(Choseong::is_compatibility_choseong('ㄱ' as u32)); // 0x3131
    assert!(Choseong::is_compatibility_choseong('ㅎ' as u32)); // 0x314E
    assert!(!Choseong::is_compatibility_choseong('ㅏ' as u32)); // 0x314F (중성)
    assert!(!Choseong::is_compatibility_choseong('가' as u32)); // 0xAC00 (완성형)
  }

  #[test]
  fn test_both_jamo_types() {
    // 조합형 초성으로 생성
    let choseong = Choseong::new(0x1112); // ᄒ
    assert_eq!(choseong.conjoining_value, 'ᄒ');
    assert_eq!(choseong.compatibility_value, 'ㅎ');

    // 호환형 초성으로 생성
    let choseong = Choseong::new(0x314E); // ㅎ
    assert_eq!(choseong.conjoining_value, 'ᄒ');
    assert_eq!(choseong.compatibility_value, 'ㅎ');
  }
}
