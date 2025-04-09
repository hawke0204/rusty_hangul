import { describe, it, expect } from "vitest";
import { Hangul } from "../index";

describe("Hangul class", () => {
	describe("disassemble method", () => {
		// 기본 한글 문자를 올바르게 분해하는지 테스트
		it("should correctly disassemble basic Hangul characters", () => {
			expect(new Hangul("안녕").disassemble()).toBe("ㅇㅏㄴㄴㅕㅇ");
			expect(new Hangul("가나다").disassemble()).toBe("ㄱㅏㄴㅏㄷㅏ");
			expect(new Hangul("한글").disassemble()).toBe("ㅎㅏㄴㄱㅡㄹ");
		});

		// 한글 분해 시 비-한글 문자가 보존되는지 테스트
		it("should preserve non-Hangul characters while disassembling Hangul", () => {
			expect(new Hangul("Hello 안녕!").disassemble()).toBe(
				"Hello ㅇㅏㄴㄴㅕㅇ!",
			);
			expect(new Hangul("123 한글 ABC").disassemble()).toBe(
				"123 ㅎㅏㄴㄱㅡㄹ ABC",
			);
		});

		// 빈 문자열이 주어졌을 때 빈 문자열을 반환하는지 테스트
		it("should return an empty string when given an empty string", () => {
			expect(new Hangul("").disassemble()).toBe("");
		});

		// 복잡한 음절을 올바르게 분해하는지 테스트
		it("should correctly disassemble complex syllables", () => {
			expect(new Hangul("꿈").disassemble()).toBe("ㄲㅜㅁ");
			expect(new Hangul("밝다").disassemble()).toBe("ㅂㅏㄹㄱㄷㅏ");
			expect(new Hangul("닭고기").disassemble()).toBe("ㄷㅏㄹㄱㄱㅗㄱㅣ");
		});

		// 공백이 있는 문자열을 올바르게 처리하는지 테스트
		it("should handle strings with spaces correctly", () => {
			expect(new Hangul("안녕 하세요").disassemble()).toBe(
				"ㅇㅏㄴㄴㅕㅇ ㅎㅏㅅㅔㅇㅛ",
			);
		});
	});

	describe("getChoseong method", () => {
		// 기본 한글 문자에서 초성을 올바르게 추출하는지 테스트
		it("should correctly extract choseong from basic Hangul characters", () => {
			expect(new Hangul("안녕").getChoseong()).toBe("ㅇㄴ");
			expect(new Hangul("가나다").getChoseong()).toBe("ㄱㄴㄷ");
			expect(new Hangul("한글").getChoseong()).toBe("ㅎㄱ");
		});

		// 초성 추출 시 비-한글 문자가 보존되는지 테스트
		it("should preserve non-Hangul characters while extracting choseong", () => {
			expect(new Hangul("Hello 안녕!").getChoseong()).toBe("Hello ㅇㄴ!");
			expect(new Hangul("123 한글 ABC").getChoseong()).toBe("123 ㅎㄱ ABC");
		});

		// 빈 문자열이 주어졌을 때 빈 문자열을 반환하는지 테스트
		it("should return an empty string when given an empty string", () => {
			expect(new Hangul("").getChoseong()).toBe("");
		});

		// 복잡한 음절에서 초성을 올바르게 추출하는지 테스트
		it("should correctly extract choseong from complex syllables", () => {
			expect(new Hangul("꿈").getChoseong()).toBe("ㄲ");
			expect(new Hangul("밝다").getChoseong()).toBe("ㅂㄷ");
			expect(new Hangul("닭고기").getChoseong()).toBe("ㄷㄱㄱ");
		});

		// 공백이 있는 문자열을 올바르게 처리하는지 테스트
		it("should handle strings with spaces correctly", () => {
			expect(new Hangul("안녕 하세요").getChoseong()).toBe("ㅇㄴ ㅎㅅㅇ");
		});
	});
});
