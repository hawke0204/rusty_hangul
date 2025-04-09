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
  fn test_is_conjoining_choseong() {
    assert!(Choseong::is_conjoining_choseong(0x1100));
    assert!(Choseong::is_conjoining_choseong(0x1112));

    assert!(Choseong::is_conjoining_choseong(CHOSEONG_BASE));
    assert!(Choseong::is_conjoining_choseong(CHOSEONG_LAST));

    assert!(!Choseong::is_conjoining_choseong(CHOSEONG_BASE - 1));
    assert!(!Choseong::is_conjoining_choseong(CHOSEONG_LAST + 1));
    assert!(!Choseong::is_conjoining_choseong(0x1113));
  }

  #[test]
  fn test_is_compatibility_choseong() {
    assert!(Choseong::is_compatibility_choseong(0x3131));
    assert!(Choseong::is_compatibility_choseong(0x314E));

    assert!(Choseong::is_compatibility_choseong(COMPAT_CHOSEONG_BASE));
    assert!(Choseong::is_compatibility_choseong(COMPAT_CHOSEONG_LAST));

    assert!(!Choseong::is_compatibility_choseong(
      COMPAT_CHOSEONG_BASE - 1
    ));
    assert!(!Choseong::is_compatibility_choseong(
      COMPAT_CHOSEONG_LAST + 1
    ));
    assert!(!Choseong::is_compatibility_choseong(0x314F));
    assert!(!Choseong::is_compatibility_choseong(0xAC00));
  }

  #[test]
  fn test_conjoining_to_compatibility_conversion() {
    assert_eq!(
      Choseong::conjoining_choseong_to_compatibility(0x1100),
      Some(0x3131)
    );
    assert_eq!(
      Choseong::conjoining_choseong_to_compatibility(0x1112),
      Some(0x314E)
    );

    assert_eq!(
      Choseong::conjoining_choseong_to_compatibility(CHOSEONG_BASE),
      Some(COMPAT_CHOSEONG_BASE)
    );
    assert_eq!(
      Choseong::conjoining_choseong_to_compatibility(CHOSEONG_LAST),
      Some(COMPAT_CHOSEONG_LAST)
    );

    assert_eq!(
      Choseong::conjoining_choseong_to_compatibility(CHOSEONG_BASE - 1),
      None
    );
    assert_eq!(
      Choseong::conjoining_choseong_to_compatibility(CHOSEONG_LAST + 1),
      None
    );
  }

  #[test]
  fn test_compatibility_to_conjoining_conversion() {
    assert_eq!(
      Choseong::compatibility_to_conjoining_choseong(0x3131),
      Some(0x1100)
    );
    assert_eq!(
      Choseong::compatibility_to_conjoining_choseong(0x314E),
      Some(0x1112)
    );

    assert_eq!(
      Choseong::compatibility_to_conjoining_choseong(COMPAT_CHOSEONG_BASE),
      Some(CHOSEONG_BASE)
    );
    assert_eq!(
      Choseong::compatibility_to_conjoining_choseong(COMPAT_CHOSEONG_LAST),
      Some(CHOSEONG_LAST)
    );

    assert_eq!(
      Choseong::compatibility_to_conjoining_choseong(COMPAT_CHOSEONG_BASE - 1),
      None
    );
    assert_eq!(
      Choseong::compatibility_to_conjoining_choseong(COMPAT_CHOSEONG_LAST + 1),
      None
    );
    assert_eq!(Choseong::compatibility_to_conjoining_choseong(0x314F), None);
  }

  #[test]
  fn test_choseong_construction_from_conjoining() {
    let choseong = Choseong::new(0x1100);
    assert_eq!(choseong.conjoining_unicode, 0x1100);
    assert_eq!(choseong.conjoining_value, 'ᄀ');
    assert_eq!(choseong.compatibility_unicode, 0x3131);
    assert_eq!(choseong.compatibility_value, 'ㄱ');

    let choseong = Choseong::new(0x1112);
    assert_eq!(choseong.conjoining_unicode, 0x1112);
    assert_eq!(choseong.conjoining_value, 'ᄒ');
    assert_eq!(choseong.compatibility_unicode, 0x314E);
    assert_eq!(choseong.compatibility_value, 'ㅎ');
  }

  #[test]
  fn test_choseong_construction_from_compatibility() {
    let choseong = Choseong::new(0x3131);
    assert_eq!(choseong.conjoining_unicode, 0x1100);
    assert_eq!(choseong.conjoining_value, 'ᄀ');
    assert_eq!(choseong.compatibility_unicode, 0x3131);
    assert_eq!(choseong.compatibility_value, 'ㄱ');

    let choseong = Choseong::new(0x314E);
    assert_eq!(choseong.conjoining_unicode, 0x1112);
    assert_eq!(choseong.conjoining_value, 'ᄒ');
    assert_eq!(choseong.compatibility_unicode, 0x314E);
    assert_eq!(choseong.compatibility_value, 'ㅎ');
  }

  #[test]
  #[should_panic(expected = "유효한 초성 유니코드가 아닙니다")]
  fn test_invalid_unicode_should_panic() {
    Choseong::new(0xAC00);
  }
}
