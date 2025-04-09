import { describe, it, expect } from "vitest";
import { hangulGetChoseong } from "../index";

describe("hangulGetChoseong function", () => {
	it("should correctly extract choseong from basic Hangul characters", () => {
		expect(hangulGetChoseong("안녕")).toBe("ㅇㄴ");
		expect(hangulGetChoseong("가나다")).toBe("ㄱㄴㄷ");
		expect(hangulGetChoseong("한글")).toBe("ㅎㄱ");
	});

	it("should preserve non-Hangul characters while extracting choseong", () => {
		expect(hangulGetChoseong("Hello 안녕!")).toBe("Hello ㅇㄴ!");
		expect(hangulGetChoseong("123 한글 ABC")).toBe("123 ㅎㄱ ABC");
	});

	it("should return an empty string when given an empty string", () => {
		expect(hangulGetChoseong("")).toBe("");
	});

	it("should correctly extract choseong from complex syllables", () => {
		expect(hangulGetChoseong("꿈")).toBe("ㄲ");
		expect(hangulGetChoseong("밝다")).toBe("ㅂㄷ");
		expect(hangulGetChoseong("닭고기")).toBe("ㄷㄱㄱ");
	});

	it("should handle strings with spaces correctly", () => {
		expect(hangulGetChoseong("안녕 하세요")).toBe("ㅇㄴ ㅎㅅㅇ");
	});
});
