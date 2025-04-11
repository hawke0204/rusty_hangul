import { describe, bench as vitestBench } from "vitest";
import { fakerKO as faker } from "@faker-js/faker";

import { Hangul } from "../../node/index";
import * as esHangul from "es-hangul";

const benchOption = {
	iterations: 10000,
	time: 0,
};

const bench = (name: string, fn: () => void, options = benchOption) =>
	vitestBench(name, fn, options);

describe("한글 이름 분석", async () => {
	const text = faker.person.fullName();

	bench("es-hangul", () => {
		esHangul.disassemble(text);
		esHangul.getChoseong(text);
	});

	bench("rusty-hangul/node", () => {
		const rustyHangul = new Hangul(text);
		rustyHangul.disassemble();
		rustyHangul.getChoseong();
	});
});

describe("한글 문장 분석", async () => {
	const text =
		"우리나라 대한민국은 오천년의 유구한 역사와 전통을 자랑하는 문화 국가입니다.";

	bench("es-hangul", () => {
		esHangul.disassemble(text);
		esHangul.getChoseong(text);
	});

	bench("rusty-hangul/node", () => {
		const rustyHangul = new Hangul(text);
		rustyHangul.disassemble();
		rustyHangul.getChoseong();
	});
});

describe("한글 문단 분석", async () => {
	const text = `우리나라 대한민국은 오천년의 유구한 역사와 전통을 자랑하는 문화 국가입니다.\n
		한글은 세계에서 가장 과학적이고 배우기 쉬운 문자로 인정받고 있습니다.\n
		컴퓨터 과학에서 알고리즘과 자료구조는 프로그램의 성능을 좌우하는 핵심 요소입니다.\n
		효율적인 알고리즘 설계는 프로그램의 속도와 메모리 사용량을 크게 개선할 수 있습니다.`;

	bench("es-hangul", () => {
		esHangul.disassemble(text);
		esHangul.getChoseong(text);
	});

	bench("rusty-hangul/node", () => {
		const rustyHangul = new Hangul(text);
		rustyHangul.disassemble();
		rustyHangul.getChoseong();
	});
});

describe("한글 긴 문단 분석", async () => {
	const text = `우리나라 대한민국은 오천년의 유구한 역사와 전통을 자랑하는 문화 국가입니다.\n
		한글은 세계에서 가장 과학적이고 배우기 쉬운 문자로 인정받고 있습니다.\n
		컴퓨터 과학에서 알고리즘과 자료구조는 프로그램의 성능을 좌우하는 핵심 요소입니다.\n
		효율적인 알고리즘 설계는 프로그램의 속도와 메모리 사용량을 크게 개선할 수 있습니다.\n
		한국의 사계절은 각기 다른 아름다움을 가지고 있습니다.\n
		봄에는 벚꽃이 흩날리고, 여름에는 푸른 바다가 반기며, 가을에는 단풍이 물들고, 겨울에는 하얀 눈이 내립니다.\n
		이러한 자연의 변화는 한국 문화와 예술에 깊은 영향을 미쳤습니다.\n
		한국의 전통 음악과 춤은 이러한 자연의 아름다움을 표현하고 있습니다.\n
		한국의 전통 의상인 한복은 자연의 색깔을 담고 있으며, 한국의 전통 음식은 자연에서 얻은 재료로 만들어집니다.`;

	bench("es-hangul", () => {
		esHangul.disassemble(text);
		esHangul.getChoseong(text);
	});

	bench("rusty-hangul/node", () => {
		const rustyHangul = new Hangul(text);
		rustyHangul.disassemble();
		rustyHangul.getChoseong();
	});
});
