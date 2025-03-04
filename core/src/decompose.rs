use crate::choseong::Choseong;
use crate::jongseong::Jongseong;
use crate::jungseong::Jungseong;
use crate::nfd::Nfd;
use crate::utils::is_complete_hangul_from_u32;

pub struct Decompose;

impl Decompose {
  pub fn decompose_to_groups(string: String) -> Vec<Vec<String>> {
    string
      .chars()
      .map(|letter| {
        let decomposed_hangul = DecomposedHangul::new(letter);

        if let Some(decomposed_hangul) = decomposed_hangul {
          let choseong = decomposed_hangul.choseong.decomposed_string;
          let jungseong = decomposed_hangul.jungseong.decomposed_string;

          let mut group = vec![choseong, jungseong];

          let jongseong = decomposed_hangul.jongseong;
          if let Some(jongseong) = jongseong {
            group.push(jongseong.decomposed_string)
          }

          return group;
        } else {
          vec![letter.to_string()]
        }
      })
      .collect()
  }

  pub fn get_choseong(string: String) -> String {
    string
      .chars()
      .map(|letter| {
        let decomposed_hangul = DecomposedHangul::new(letter);
        if let Some(decomposed_hangul) = decomposed_hangul {
          let choseong = decomposed_hangul.choseong.value;
          return choseong;
        } else {
          return letter;
        }
      })
      .collect()
  }

  pub fn has_batchim(mut string: String) -> bool {
    let maybe_last_char = string.pop();

    if let Some(last_char) = maybe_last_char {
      let decomposed_hangul = DecomposedHangul::new(last_char);
      if let Some(decomposed_hangul) = decomposed_hangul {
        decomposed_hangul.jongseong.is_some()
      } else {
        false
      }
    } else {
      false
    }
  }
}

#[derive(Debug)]
struct DecomposedHangul {
  choseong: Choseong,
  jungseong: Jungseong,
  jongseong: Option<Jongseong>,
}

impl DecomposedHangul {
  fn new(letter: char) -> Option<DecomposedHangul> {
    Self::new_inner(letter)
  }

  fn new_inner(letter: char) -> Option<DecomposedHangul> {
    let letter = letter as u32;

    if !is_complete_hangul_from_u32(letter) {
      return None;
    }

    let Nfd(choseong_code, jungseong_code, jongseong_code) = Nfd::normalize(letter);
    let choseong = Choseong::new(choseong_code);
    let jungseong = Jungseong::new(jungseong_code);

    let mut jongseong = None;
    if let Some(jongseong_code) = jongseong_code {
      jongseong = Jongseong::maybe_new(jongseong_code);
    }

    Some(Self {
      choseong,
      jungseong,
      jongseong,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_choseong() {
    let testcase = "버그가 싫다.".to_string();
    let result = Decompose::get_choseong(testcase);
    println!("{:?}", result);
  }

  #[test]
  fn test_decompose_to_groups() {
    let testcase = "버그가 싫다".to_string();
    let result = Decompose::decompose_to_groups(testcase);
    let expected = vec![
      vec!["ᄇ", "ᅥ"],
      vec!["ᄀ", "ᅳ"],
      vec!["ᄀ", "ᅡ"],
      vec![" "],
      vec!["ᄉ", "ᅵ", "ᆯᇂ"],
      vec!["ᄃ", "ᅡ"],
    ];
    assert_eq!(result, expected);
  }

  #[test]
  fn test_decomposed_hangul() {
    let result = DecomposedHangul::new('각');
    if let Some(decomposed_hangul) = result {
      let choseong = decomposed_hangul.choseong.unicode;
      let jungseong = decomposed_hangul.jungseong.unicode;
      let jongseong = decomposed_hangul.jongseong.unwrap().unicode;

      assert_eq!(choseong, 0x1100); // 'ㄱ'
      assert_eq!(jungseong, 0x1161); // 'ㅏ'
      assert_eq!(jongseong, 0x11A8); // 'ㄱ'
      assert_ne!(choseong, jongseong); // 초성과 종성의 'ㄱ' 유니코드는 다름
    }
  }
}
