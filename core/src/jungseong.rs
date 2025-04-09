use crate::utils::is_compatibility_jamo;

// 조합형 중성 범위
const JUNGSEONG_BASE: u32 = 0x1161;
const JUNGSEONG_LAST: u32 = 0x1175;

// 호환형 중성 범위
const COMPAT_JUNGSEONG_BASE: u32 = 0x314F;
const COMPAT_JUNGSEONG_LAST: u32 = 0x3163;

// 호환형 중성 매핑 테이블
const COMPATIBILITY_JUNGSEONG_MAPPING: [u32; 21] = [
  0x314F, 0x3150, 0x3151, 0x3152, 0x3153, 0x3154, 0x3155, 0x3156, 0x3157, 0x3158, 0x3159, 0x315A,
  0x315B, 0x315C, 0x315D, 0x315E, 0x315F, 0x3160, 0x3161, 0x3162, 0x3163,
];

#[derive(Debug)]
pub struct Jungseong {
  pub conjoining_value: char,
  pub conjoining_unicode: u32,
  pub compatibility_value: char,
  pub compatibility_unicode: u32,
}

impl Jungseong {
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
    if Self::is_conjoining_jungseong(unicode) {
      let conjoining_jamo = unicode;
      let compatibility_jamo = Self::conjoining_jungseong_to_compatibility(unicode)
        .expect("조합형 자모를 호환형으로 변환하는데 실패했습니다");
      return (conjoining_jamo, compatibility_jamo);
    }

    if Self::is_compatibility_jungseong(unicode) {
      let conjoining_jamo = Self::compatibility_to_conjoining_jungseong(unicode)
        .expect("호환형 자모를 조합형으로 변환하는데 실패했습니다");
      let compatibility_jamo = unicode;
      return (conjoining_jamo, compatibility_jamo);
    }

    panic!("유효한 중성 유니코드가 아닙니다: {}", unicode)
  }

  // 조합형 중성 확인
  pub fn is_conjoining_jungseong(jungseong_code: u32) -> bool {
    JUNGSEONG_BASE <= jungseong_code && jungseong_code <= JUNGSEONG_LAST
  }

  // 호환형 중성 확인
  pub fn is_compatibility_jungseong(unicode: u32) -> bool {
    COMPAT_JUNGSEONG_BASE <= unicode && unicode <= COMPAT_JUNGSEONG_LAST
  }

  fn compatibility_to_conjoining_jungseong(compat: u32) -> Option<u32> {
    if !is_compatibility_jamo(compat) || !Self::is_compatibility_jungseong(compat) {
      return None;
    }

    COMPATIBILITY_JUNGSEONG_MAPPING
      .iter()
      .position(|&x| x == compat)
      .map(|i| 0x1161 + i as u32)
  }

  fn conjoining_jungseong_to_compatibility(jungseong_code: u32) -> Option<u32> {
    if !Self::is_conjoining_jungseong(jungseong_code) {
      return None;
    }

    let offset = jungseong_code - 0x1161;
    COMPATIBILITY_JUNGSEONG_MAPPING
      .get(offset as usize)
      .copied()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_conjoining_jungseong() {
    assert!(Jungseong::is_conjoining_jungseong(0x1161));
    assert!(Jungseong::is_conjoining_jungseong(0x1175));

    assert!(Jungseong::is_conjoining_jungseong(JUNGSEONG_BASE));
    assert!(Jungseong::is_conjoining_jungseong(JUNGSEONG_LAST));

    assert!(!Jungseong::is_conjoining_jungseong(0x1160));
    assert!(!Jungseong::is_conjoining_jungseong(0x1176));
    assert!(!Jungseong::is_conjoining_jungseong(0x314F));
  }

  #[test]
  fn test_is_compatibility_jungseong() {
    assert!(Jungseong::is_compatibility_jungseong(0x314F));
    assert!(Jungseong::is_compatibility_jungseong(0x3163));

    assert!(Jungseong::is_compatibility_jungseong(COMPAT_JUNGSEONG_BASE));
    assert!(Jungseong::is_compatibility_jungseong(COMPAT_JUNGSEONG_LAST));

    assert!(!Jungseong::is_compatibility_jungseong(0x314E));
    assert!(!Jungseong::is_compatibility_jungseong(0x3164));
    assert!(!Jungseong::is_compatibility_jungseong(0x1161));
  }

  #[test]
  fn test_compatibility_to_conjoining_jungseong() {
    assert_eq!(
      Jungseong::compatibility_to_conjoining_jungseong(0x314F),
      Some(0x1161)
    );
    assert_eq!(
      Jungseong::compatibility_to_conjoining_jungseong(0x3163),
      Some(0x1175)
    );

    assert_eq!(
      Jungseong::compatibility_to_conjoining_jungseong(COMPAT_JUNGSEONG_BASE),
      Some(JUNGSEONG_BASE)
    );
    assert_eq!(
      Jungseong::compatibility_to_conjoining_jungseong(COMPAT_JUNGSEONG_LAST),
      Some(JUNGSEONG_LAST)
    );

    assert_eq!(
      Jungseong::compatibility_to_conjoining_jungseong(0x1161),
      None
    );
    assert_eq!(
      Jungseong::compatibility_to_conjoining_jungseong(0x3164),
      None
    );
  }

  #[test]
  fn test_conjoining_jungseong_to_compatibility() {
    assert_eq!(
      Jungseong::conjoining_jungseong_to_compatibility(0x1161),
      Some(0x314F)
    );
    assert_eq!(
      Jungseong::conjoining_jungseong_to_compatibility(0x1175),
      Some(0x3163)
    );

    assert_eq!(
      Jungseong::conjoining_jungseong_to_compatibility(JUNGSEONG_BASE),
      Some(COMPAT_JUNGSEONG_BASE)
    );
    assert_eq!(
      Jungseong::conjoining_jungseong_to_compatibility(JUNGSEONG_LAST),
      Some(COMPAT_JUNGSEONG_LAST)
    );

    assert_eq!(
      Jungseong::conjoining_jungseong_to_compatibility(0x314F),
      None
    );
    assert_eq!(
      Jungseong::conjoining_jungseong_to_compatibility(0x1160),
      None
    );
    assert_eq!(
      Jungseong::conjoining_jungseong_to_compatibility(0x1176),
      None
    );
  }

  #[test]
  fn test_jungseong_new() {
    let jungseong = Jungseong::new(0x1161);
    assert_eq!(jungseong.conjoining_unicode, 0x1161);
    assert_eq!(jungseong.conjoining_value, 'ᅡ');
    assert_eq!(jungseong.compatibility_unicode, 0x314F);
    assert_eq!(jungseong.compatibility_value, 'ㅏ');

    let jungseong = Jungseong::new(0x314F);
    assert_eq!(jungseong.conjoining_unicode, 0x1161);
    assert_eq!(jungseong.conjoining_value, 'ᅡ');
    assert_eq!(jungseong.compatibility_unicode, 0x314F);
    assert_eq!(jungseong.compatibility_value, 'ㅏ');
  }

  #[test]
  #[should_panic(expected = "유효한 중성 유니코드가 아닙니다")]
  fn test_jungseong_invalid_unicode() {
    Jungseong::new(0x3131);
  }

  #[test]
  fn test_jungseong_conversion() {
    let jungseong = Jungseong::new('ᅡ' as u32);
    assert_eq!(jungseong.conjoining_unicode, 0x1161);
    assert_eq!(jungseong.conjoining_value, 'ᅡ');
    assert_eq!(jungseong.compatibility_unicode, 0x314F);
    assert_eq!(jungseong.compatibility_value, 'ㅏ');
  }

  #[test]
  fn test_jungseong_from_compatibility() {
    let jungseong = Jungseong::new('ㅏ' as u32);
    assert_eq!(jungseong.conjoining_unicode, 0x1161);
    assert_eq!(jungseong.conjoining_value, 'ᅡ');
    assert_eq!(jungseong.compatibility_unicode, 0x314F);
    assert_eq!(jungseong.compatibility_value, 'ㅏ');
  }
}
