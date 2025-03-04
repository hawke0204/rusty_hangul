mod choseong;
mod decompose;
mod jongseong;
mod jungseong;
mod nfd;
mod utils;

#[cfg(test)]
mod tests {
  use crate::decompose::Decompose;

  #[test]
  fn test_decompose() {
    let testcase = "안녕하세요.";
    let result = Decompose::decompose_to_groups(testcase.to_string());

    assert_eq!(
      result,
      vec![
        vec!["ᄋ", "ᅡ", "ᆫ"],
        vec!["ᄂ", "ᅧ", "ᆼ"],
        vec!["ᄒ", "ᅡ"],
        vec!["ᄉ", "ᅦ"],
        vec!["ᄋ", "ᅭ"],
        vec!["."]
      ]
    )
  }
}
