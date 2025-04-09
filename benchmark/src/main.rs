use hangul::Hangul;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Instant;

fn main() {
  println!("\n🚀 Starting Rusty Hangul Benchmark");

  let sentences = prepare_korean_sentences(1000000);

  benchmark_decompose(&sentences);

  println!("\n✅ Benchmark Complete\n");
}

fn prepare_korean_sentences(count: usize) -> Vec<String> {
  let sample_sentences = vec![
    "안녕하세요 반갑습니다".to_string(),
    "한글은 세계에서 가장 과학적인 문자입니다".to_string(),
    "오늘은 날씨가 정말 좋네요".to_string(),
    "프로그래밍 언어로 한글 처리하기".to_string(),
    "러스트는 안전하고 빠른 언어입니다".to_string(),
    "벤치마크 테스트를 진행합니다".to_string(),
    "한글날은 10월 9일입니다".to_string(),
    "세종대왕은 한글을 창제하셨습니다".to_string(),
    "가나다라마바사아자차카타파하".to_string(),
    "동해물과 백두산이 마르고 닳도록".to_string(),
    "우리나라 대한민국은 오천년의 유구한 역사와 전통을 자랑하는 문화 국가입니다. 한글은 세계에서 가장 과학적이고 배우기 쉬운 문자로 인정받고 있습니다.".to_string(),
    "컴퓨터 과학에서 알고리즘과 자료구조는 프로그램의 성능을 좌우하는 핵심 요소입니다. 효율적인 알고리즘 설계는 프로그램의 속도와 메모리 사용량을 크게 개선할 수 있습니다.".to_string(),
    "한국의 사계절은 각기 다른 아름다움을 가지고 있습니다. 봄에는 벚꽃이 흩날리고, 여름에는 푸른 바다가 반기며, 가을에는 단풍이 물들고, 겨울에는 하얀 눈이 내립니다.".to_string(),
    "인공지능 기술의 발전으로 우리의 일상생활은 크게 변화하고 있습니다. 음성 인식, 자연어 처리, 컴퓨터 비전 등 다양한 분야에서 혁신적인 기술이 등장하고 있으며, 이는 미래 사회의 모습을 바꿔놓을 것입니다.".to_string(),
    "대한민국은 IT 강국으로서 세계 최고 수준의 인터넷 인프라를 갖추고 있으며, 많은 기업들이 혁신적인 기술과 서비스를 개발하여 글로벌 시장에서 경쟁력을 유지하고 있습니다.".to_string(),
  ];

  let mut result = Vec::with_capacity(count);

  for _ in 0..count {
    let idx = rand::random::<usize>() % sample_sentences.len();
    result.push(sample_sentences[idx].clone());
  }

  result
}

fn benchmark_decompose(sentences: &[String]) {
  println!("\n[Disassemble Benchmark]");

  let start = Instant::now();

  let pb = ProgressBar::new(sentences.len() as u64);

  pb.set_style(
    ProgressStyle::default_bar()
      .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
      .unwrap(),
  );

  for sentence in sentences {
    let hangul = Hangul::new(sentence);
    let _ = hangul.disassemble();
    pb.inc(1);
  }

  pb.finish();

  let duration = start.elapsed();

  println!(" - Number of sentences processed: {}", sentences.len());

  println!(" - Total time: {:.2?}", duration);
}
