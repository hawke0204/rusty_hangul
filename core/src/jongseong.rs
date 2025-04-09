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

  pub fn is_conjoining_jongseong(jongseong_code: u32) -> bool {
    JONGSEONG_BASE <= jongseong_code && jongseong_code <= JONGSEONG_LAST
  }

  pub fn is_compatibility_jongseong(unicode: u32) -> bool {
    COMPAT_JONGSEONG_BASE <= unicode && unicode <= COMPAT_JONGSEONG_LAST
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

  fn compatibility_to_conjoining_jongseong(compat: u32) -> Option<u32> {
    if (!is_compatibility_jamo(compat)) || (!Self::is_compatibility_jongseong(compat)) {
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

  pub fn is_complex_jongseong(&self) -> bool {
    matches!(
      self.compatibility_value,
      'ㄳ' | 'ㄵ' | 'ㄶ' | 'ㄺ' | 'ㄻ' | 'ㄼ' | 'ㄽ' | 'ㄾ' | 'ㄿ' | 'ㅀ' | 'ㅄ'
    )
  }

  pub fn decompose_complex_jongseong(&self) -> Vec<char> {
    match self.compatibility_value {
      'ㄳ' => vec!['ㄱ', 'ㅅ'],
      'ㄵ' => vec!['ㄴ', 'ㅈ'],
      'ㄶ' => vec!['ㄴ', 'ㅎ'],
      'ㄺ' => vec!['ㄹ', 'ㄱ'],
      'ㄻ' => vec!['ㄹ', 'ㅁ'],
      'ㄼ' => vec!['ㄹ', 'ㅂ'],
      'ㄽ' => vec!['ㄹ', 'ㅅ'],
      'ㄾ' => vec!['ㄹ', 'ㅌ'],
      'ㄿ' => vec!['ㄹ', 'ㅍ'],
      'ㅀ' => vec!['ㄹ', 'ㅎ'],
      'ㅄ' => vec!['ㅂ', 'ㅅ'],
      _ => vec![self.compatibility_value],
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_conjoining_jongseong() {
    assert!(Jongseong::is_conjoining_jongseong(0x11A8));
    assert!(Jongseong::is_conjoining_jongseong(0x11B8));
    assert!(Jongseong::is_conjoining_jongseong(0x11C2));

    assert!(!Jongseong::is_conjoining_jongseong(0x11A7));
    assert!(!Jongseong::is_conjoining_jongseong(0x11C3));
    assert!(!Jongseong::is_conjoining_jongseong(0x3131));
  }

  #[test]
  fn test_is_compatibility_jongseong() {
    assert!(Jongseong::is_compatibility_jongseong(0x3131));
    assert!(Jongseong::is_compatibility_jongseong(0x3137));
    assert!(Jongseong::is_compatibility_jongseong(0x314E));

    assert!(!Jongseong::is_compatibility_jongseong(0x3130));
    assert!(!Jongseong::is_compatibility_jongseong(0x314F));
    assert!(!Jongseong::is_compatibility_jongseong(0x11A8));
  }

  #[test]
  fn test_compatibility_to_conjoining_jongseong() {
    assert_eq!(
      Jongseong::compatibility_to_conjoining_jongseong(0x3131),
      Some(0x11A8)
    );
    assert_eq!(
      Jongseong::compatibility_to_conjoining_jongseong(0x3134),
      Some(0x11AB)
    );
    assert_eq!(
      Jongseong::compatibility_to_conjoining_jongseong(0x314E),
      Some(0x11C2)
    );

    assert_eq!(
      Jongseong::compatibility_to_conjoining_jongseong(0x3130),
      None
    );
    assert_eq!(
      Jongseong::compatibility_to_conjoining_jongseong(0x314F),
      None
    );
    assert_eq!(
      Jongseong::compatibility_to_conjoining_jongseong(0x11A8),
      None
    );
  }

  #[test]
  fn test_conjoining_jongseong_to_compatibility() {
    assert_eq!(
      Jongseong::conjoining_jongseong_to_compatibility(0x11A8),
      Some(0x3131)
    );
    assert_eq!(
      Jongseong::conjoining_jongseong_to_compatibility(0x11AB),
      Some(0x3134)
    );
    assert_eq!(
      Jongseong::conjoining_jongseong_to_compatibility(0x11C2),
      Some(0x314E)
    );

    assert_eq!(
      Jongseong::conjoining_jongseong_to_compatibility(0x11A7),
      None
    );
    assert_eq!(
      Jongseong::conjoining_jongseong_to_compatibility(0x11C3),
      None
    );
    assert_eq!(
      Jongseong::conjoining_jongseong_to_compatibility(0x3131),
      None
    );
  }

  #[test]
  fn test_new_from_conjoining_jamo() {
    let jongseong = Jongseong::new(0x11A8);
    assert_eq!(jongseong.conjoining_unicode, 0x11A8);
    assert_eq!(jongseong.conjoining_value, 'ᆨ');
    assert_eq!(jongseong.compatibility_unicode, 0x3131);
    assert_eq!(jongseong.compatibility_value, 'ㄱ');

    let jongseong = Jongseong::new(0x11AC);
    assert_eq!(jongseong.conjoining_unicode, 0x11AC);
    assert_eq!(jongseong.conjoining_value, 'ᆬ');
    assert_eq!(jongseong.compatibility_unicode, 0x3135);
    assert_eq!(jongseong.compatibility_value, 'ㄵ');
  }

  #[test]
  fn test_new_from_compatibility_jamo() {
    let jongseong = Jongseong::new(0x3131);
    assert_eq!(jongseong.conjoining_unicode, 0x11A8);
    assert_eq!(jongseong.conjoining_value, 'ᆨ');
    assert_eq!(jongseong.compatibility_unicode, 0x3131);
    assert_eq!(jongseong.compatibility_value, 'ㄱ');

    let jongseong = Jongseong::new(0x3135);
    assert_eq!(jongseong.conjoining_unicode, 0x11AC);
    assert_eq!(jongseong.conjoining_value, 'ᆬ');
    assert_eq!(jongseong.compatibility_unicode, 0x3135);
    assert_eq!(jongseong.compatibility_value, 'ㄵ');
  }

  #[test]
  fn test_is_complex_jongseong() {
    assert!(Jongseong::new(0x3133).is_complex_jongseong());
    assert!(Jongseong::new(0x3135).is_complex_jongseong());
    assert!(Jongseong::new(0x3136).is_complex_jongseong());
    assert!(Jongseong::new(0x313A).is_complex_jongseong());
    assert!(Jongseong::new(0x313B).is_complex_jongseong());
    assert!(Jongseong::new(0x313C).is_complex_jongseong());
    assert!(Jongseong::new(0x313D).is_complex_jongseong());
    assert!(Jongseong::new(0x313E).is_complex_jongseong());
    assert!(Jongseong::new(0x313F).is_complex_jongseong());
    assert!(Jongseong::new(0x3140).is_complex_jongseong());
    assert!(Jongseong::new(0x3144).is_complex_jongseong());

    assert!(!Jongseong::new(0x3131).is_complex_jongseong());
    assert!(!Jongseong::new(0x3134).is_complex_jongseong());
    assert!(!Jongseong::new(0x3139).is_complex_jongseong());
  }

  #[test]
  fn test_decompose_complex_jongseong() {
    assert_eq!(
      Jongseong::new(0x3133).decompose_complex_jongseong(),
      vec!['ㄱ', 'ㅅ']
    );
    assert_eq!(
      Jongseong::new(0x3135).decompose_complex_jongseong(),
      vec!['ㄴ', 'ㅈ']
    );
    assert_eq!(
      Jongseong::new(0x3136).decompose_complex_jongseong(),
      vec!['ㄴ', 'ㅎ']
    );
    assert_eq!(
      Jongseong::new(0x313A).decompose_complex_jongseong(),
      vec!['ㄹ', 'ㄱ']
    );
    assert_eq!(
      Jongseong::new(0x313B).decompose_complex_jongseong(),
      vec!['ㄹ', 'ㅁ']
    );
    assert_eq!(
      Jongseong::new(0x313C).decompose_complex_jongseong(),
      vec!['ㄹ', 'ㅂ']
    );
    assert_eq!(
      Jongseong::new(0x313D).decompose_complex_jongseong(),
      vec!['ㄹ', 'ㅅ']
    );
    assert_eq!(
      Jongseong::new(0x313E).decompose_complex_jongseong(),
      vec!['ㄹ', 'ㅌ']
    );
    assert_eq!(
      Jongseong::new(0x313F).decompose_complex_jongseong(),
      vec!['ㄹ', 'ㅍ']
    );
    assert_eq!(
      Jongseong::new(0x3140).decompose_complex_jongseong(),
      vec!['ㄹ', 'ㅎ']
    );
    assert_eq!(
      Jongseong::new(0x3144).decompose_complex_jongseong(),
      vec!['ㅂ', 'ㅅ']
    );

    assert_eq!(
      Jongseong::new(0x3131).decompose_complex_jongseong(),
      vec!['ㄱ']
    );
    assert_eq!(
      Jongseong::new(0x3134).decompose_complex_jongseong(),
      vec!['ㄴ']
    );
    assert_eq!(
      Jongseong::new(0x3139).decompose_complex_jongseong(),
      vec!['ㄹ']
    );
  }

  #[test]
  #[should_panic(expected = "유효한 종성 유니코드가 아닙니다")]
  fn test_invalid_unicode() {
    Jongseong::new(0x1100);
  }
}
