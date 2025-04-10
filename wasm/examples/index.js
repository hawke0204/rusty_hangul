import init, { disassemble, getChoseong } from "../pkg/hangul.js";

async function initWasm() {
	try {
		await init();
		console.log("WASM 모듈이 초기화되었습니다!");

		const testText = "안녕하세요";
		console.log(`테스트 텍스트: "${testText}"`);
		console.log(`분해 결과: ${disassemble(testText, true)}`);
		console.log(`초성 추출: ${getChoseong(testText)}`);

		setupEventListeners();
	} catch (error) {
		console.error("WASM 초기화 중 오류 발생:", error);
	}
}

function setupEventListeners() {
	const disassembleBtn = document.getElementById("disassembleBtn");
	const choseongBtn = document.getElementById("choseongBtn");
	const inputText = document.getElementById("inputText");
	const resultOutput = document.getElementById("resultOutput");

	disassembleBtn.addEventListener("click", () => {
		const text = inputText.value;
		if (!text) {
			resultOutput.textContent = "텍스트를 입력해주세요.";
			return;
		}

		const disassembled = disassemble(text, true);

		resultOutput.innerHTML = `
      <p><strong>분해 결과:</strong> ${disassembled}</p>
    `;
	});

	choseongBtn.addEventListener("click", () => {
		const text = inputText.value;
		if (!text) {
			resultOutput.textContent = "텍스트를 입력해주세요.";
			return;
		}

		const choseong = getChoseong(text);

		resultOutput.innerHTML = `
      <p><strong>초성 추출 결과:</strong> ${choseong}</p>
      <p><strong>원본 텍스트:</strong> ${text}</p>
    `;
	});
}

initWasm();
