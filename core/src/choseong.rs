use crate::utils::is_conjoining_choseong_from_u32;

#[derive(Debug)]
pub struct Choseong {
  pub value: char,
  pub unicode: u32,
  pub decomposed: Vec<u32>,
  pub decomposed_string: String,
}

impl Choseong {
  pub fn new(unicode: u32) -> Self {
    Self::new_from_u32(unicode)
  }

  fn new_from_u32(unicode: u32) -> Self {
    if !is_conjoining_choseong_from_u32(unicode) {
      panic!()
    }

    let value = unsafe { std::char::from_u32_unchecked(unicode) };

    Self {
      value,
      unicode,
      decomposed: vec![unicode],
      decomposed_string: value.to_string(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_choseong() {
    use crate::nfd::Nfd;

    let letter = '궑';
    let Nfd(choseong_code, _, _) = Nfd::normalize(letter as u32).unwrap();
    let choseong = Choseong::new_from_u32(choseong_code);

    assert_eq!(choseong.unicode, 0x1100);
    assert_eq!(
      choseong
        .decomposed_string
        .chars()
        .map(|c| c as u32)
        .collect::<Vec<u32>>(),
      vec![0x1100]
    )
  }
}
