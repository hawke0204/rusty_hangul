use crate::utils::is_jungseong_from_u32;

#[derive(Debug)]
pub struct Jungseong {
  pub value: char,
  pub unicode: u32,
  pub decomposed: Vec<u32>,
  pub decomposed_string: String,
}

impl Jungseong {
  pub fn new(unicode: u32) -> Self {
    Self::new_from_u32(unicode)
  }

  fn new_from_u32(unicode: u32) -> Self {
    if !is_jungseong_from_u32(unicode) {
      panic!()
    }

    let value = unsafe { std::char::from_u32_unchecked(unicode) };
    let decomposed = Self::decompose_jungseong_from_u32(&unicode);
    let decomposed_string = decomposed
      .clone()
      .into_iter()
      .map(|code| {
        let character = char::from_u32(code).unwrap();
        character.to_string()
      })
      .collect::<Vec<String>>()
      .join("");

    Self {
      value,
      unicode,
      decomposed,
      decomposed_string,
    }
  }

  fn decompose_jungseong_from_u32(jungseong_code: &u32) -> Vec<u32> {
    match jungseong_code {
      0x1161 => vec![0x1161],         // "ㅏ"
      0x1162 => vec![0x1162],         // "ㅐ"
      0x1163 => vec![0x1163],         // "ㅑ"
      0x1164 => vec![0x1164],         // "ㅒ"
      0x1165 => vec![0x1165],         // "ㅓ"
      0x1166 => vec![0x1166],         // "ㅔ"
      0x1167 => vec![0x1167],         // "ㅕ"
      0x1168 => vec![0x1168],         // "ㅖ"
      0x1169 => vec![0x1169],         // "ㅗ"
      0x116A => vec![0x1169, 0x1161], // "ㅗㅏ"
      0x116B => vec![0x1169, 0x1162], // "ㅗㅐ"
      0x116C => vec![0x1169, 0x1175], // "ㅗㅣ"
      0x116D => vec![0x116D],         // "ㅛ",
      0x116E => vec![0x116E],         // "ㅜ",
      0x116F => vec![0x116E, 0x1165], // "ㅜㅓ",
      0x1170 => vec![0x116E, 0x1166], // "ㅜㅔ",
      0x1171 => vec![0x116E, 0x1175], // "ㅜㅣ",
      0x1172 => vec![0x1172],         // "ㅠ",
      0x1173 => vec![0x1173],         // "ㅡ",
      0x1174 => vec![0x1173, 0x1175], // "ㅡㅣ",
      0x1175 => vec![0x1175],         // "ㅣ",
      _ => unreachable!(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::nfd::Nfd;

  #[test]
  fn test_jungseong() {
    let letter = '궐';
    let Nfd(_, jungseong_code, _) = Nfd::normalize(letter as u32);
    let jungseong = Jungseong::new_from_u32(jungseong_code);

    assert_eq!(
      jungseong
        .decomposed_string
        .chars()
        .map(|c| c as u32)
        .collect::<Vec<u32>>(),
      "ㅜㅓ"
        .chars()
        // Mac OS에서 글자 하나의 자음마다 0x1FEE 값 만큼 더해지는 버그가 있다.
        .map(|c| if is_jungseong_from_u32(c as u32) {
          c as u32
        } else {
          c as u32 - 0x1FEE
        })
        .collect::<Vec<u32>>()
    )
  }
}
