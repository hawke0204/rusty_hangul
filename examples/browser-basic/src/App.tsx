import { useEffect, useState } from "react";
import "./App.css";
import initHangul, { disassemble, getChoseong } from "../../../wasm/pkg/hangul";

type Wasm = {
	disassemble: (text: string) => string;
	getChoseong: (text: string) => string;
};

function App() {
	const [inputText, setInputText] = useState<string>("안녕하세요");
	const [result, setResult] = useState<string>("");
	const [wasm] = useState<Wasm>({
		disassemble,
		getChoseong,
	});

	useEffect(() => {
		initHangul();
	}, []);

	return (
		<div className="container">
			<h1>한글 WASM 데모</h1>

			<div className="form-group">
				<label htmlFor="inputText">한글 입력:</label>
				<input
					type="text"
					id="inputText"
					value={inputText}
					onChange={(e) => setInputText(e.target.value)}
				/>
			</div>

			<div className="button-group">
				<button
					type="button"
					onClick={() => {
						if (inputText) {
							const disassembled = wasm?.disassemble(inputText);
							setResult(disassembled);
						}
					}}
				>
					분해하기
				</button>
				<button
					type="button"
					onClick={() => {
						if (inputText) {
							const choseong = wasm?.getChoseong(inputText);
							setResult(choseong);
						}
					}}
				>
					초성 추출
				</button>
			</div>

			<div className="result">
				<h3>결과:</h3>
				<div>{result}</div>
			</div>
		</div>
	);
}

export default App;
