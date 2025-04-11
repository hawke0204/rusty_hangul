use criterion::{criterion_group, criterion_main, Criterion};
use hangul::Hangul;
use rand::{rngs::StdRng, Rng, SeedableRng};

fn get_test_cases() -> Vec<(String, String)> {
  let mut rng = StdRng::seed_from_u64(42);
  let random_name = format!(
    "{}{}{}",
    ["김", "이", "박", "최", "정", "강", "조", "윤", "장", "임"][rng.gen_range(0..10)],
    ["민", "준", "서", "도", "윤", "지", "현", "예", "성", "수"][rng.gen_range(0..10)],
    ["우", "준", "호", "아", "은", "지", "현", "빈", "서", "영"][rng.gen_range(0..10)],
  );

  vec![
        (
            "한글 이름 분석".to_string(),
            random_name,
        ),
        (
            "한글 문장 분석".to_string(),
            "우리나라 대한민국은 오천년의 유구한 역사와 전통을 자랑하는 문화 국가입니다.".to_string(),
        ),
        (
            "한글 문단 분석".to_string(),
            "우리나라 대한민국은 오천년의 유구한 역사와 전통을 자랑하는 문화 국가입니다.
한글은 세계에서 가장 과학적이고 배우기 쉬운 문자로 인정받고 있습니다.
컴퓨터 과학에서 알고리즘과 자료구조는 프로그램의 성능을 좌우하는 핵심 요소입니다.
효율적인 알고리즘 설계는 프로그램의 속도와 메모리 사용량을 크게 개선할 수 있습니다.".to_string(),
        ),
        (
            "한글 긴 문단 분석".to_string(),
            "우리나라 대한민국은 오천년의 유구한 역사와 전통을 자랑하는 문화 국가입니다.
한글은 세계에서 가장 과학적이고 배우기 쉬운 문자로 인정받고 있습니다.
컴퓨터 과학에서 알고리즘과 자료구조는 프로그램의 성능을 좌우하는 핵심 요소입니다.
효율적인 알고리즘 설계는 프로그램의 속도와 메모리 사용량을 크게 개선할 수 있습니다.
한국의 사계절은 각기 다른 아름다움을 가지고 있습니다.
봄에는 벚꽃이 흩날리고, 여름에는 푸른 바다가 반기며, 가을에는 단풍이 물들고, 겨울에는 하얀 눈이 내립니다.
이러한 자연의 변화는 한국 문화와 예술에 깊은 영향을 미쳤습니다.
한국의 전통 음악과 춤은 이러한 자연의 아름다움을 표현하고 있습니다.
한국의 전통 의상인 한복은 자연의 색깔을 담고 있으며, 한국의 전통 음식은 자연에서 얻은 재료로 만들어집니다.".to_string(),
        ),
    ]
}

// 한글 이름 분석 벤치마크
fn bench_name_analysis(c: &mut Criterion) {
  let mut group = c.benchmark_group("한글 이름 분석");

  let text = &get_test_cases()[0].1;

  group.sample_size(10000);

  group.bench_function("rusty-hangul", |b| {
    b.iter(|| {
      let hangul = Hangul::new(text);
      let _ = hangul.disassemble();
      let _ = hangul.get_choseong();
    })
  });

  group.finish();
}

// 한글 문장 분석 벤치마크
fn bench_sentence_analysis(c: &mut Criterion) {
  let mut group = c.benchmark_group("한글 문장 분석");

  let text = &get_test_cases()[1].1;

  group.sample_size(10000);

  group.bench_function("rusty-hangul", |b| {
    b.iter(|| {
      let hangul = Hangul::new(text);
      let _ = hangul.disassemble();
      let _ = hangul.get_choseong();
    })
  });

  group.finish();
}

// 한글 문단 분석 벤치마크
fn bench_paragraph_analysis(c: &mut Criterion) {
  let mut group = c.benchmark_group("한글 문단 분석");

  let text = &get_test_cases()[2].1;

  group.sample_size(10000);

  group.bench_function("rusty-hangul", |b| {
    b.iter(|| {
      let hangul = Hangul::new(text);
      let _ = hangul.disassemble();
      let _ = hangul.get_choseong();
    })
  });

  group.finish();
}

// 한글 긴 문단 분석 벤치마크
fn bench_long_paragraph_analysis(c: &mut Criterion) {
  let mut group = c.benchmark_group("한글 긴 문단 분석");

  let text = &get_test_cases()[3].1;

  group.sample_size(10000);

  group.bench_function("rusty-hangul", |b| {
    b.iter(|| {
      let hangul = Hangul::new(text);
      let _ = hangul.disassemble();
      let _ = hangul.get_choseong();
    })
  });

  group.finish();
}

criterion_group!(
  benches,
  bench_name_analysis,
  bench_sentence_analysis,
  bench_paragraph_analysis,
  bench_long_paragraph_analysis
);
criterion_main!(benches);
