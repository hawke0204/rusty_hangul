import { describe, it, expect } from "vitest";
import { hangulDisassemble } from "../index";

describe("hangulDisassemble function", () => {
	it("should correctly disassemble basic Hangul characters", () => {
		expect(hangulDisassemble("안녕")).toBe("ㅇㅏㄴㄴㅕㅇ");
		expect(hangulDisassemble("가나다")).toBe("ㄱㅏㄴㅏㄷㅏ");
		expect(hangulDisassemble("한글")).toBe("ㅎㅏㄴㄱㅡㄹ");
	});

	it("should preserve non-Hangul characters while disassembling Hangul", () => {
		expect(hangulDisassemble("Hello 안녕!")).toBe("Hello ㅇㅏㄴㄴㅕㅇ!");
		expect(hangulDisassemble("123 한글 ABC")).toBe("123 ㅎㅏㄴㄱㅡㄹ ABC");
	});

	it("should return an empty string when given an empty string", () => {
		expect(hangulDisassemble("")).toBe("");
	});

	it("should correctly disassemble complex syllables", () => {
		expect(hangulDisassemble("꿈")).toBe("ㄲㅜㅁ");
		expect(hangulDisassemble("밝다")).toBe("ㅂㅏㄹㄱㄷㅏ");
		expect(hangulDisassemble("닭고기")).toBe("ㄷㅏㄹㄱㄱㅗㄱㅣ");
	});

	it("should handle strings with spaces correctly", () => {
		expect(hangulDisassemble("안녕 하세요")).toBe("ㅇㅏㄴㄴㅕㅇ ㅎㅏㅅㅔㅇㅛ");
	});
});
