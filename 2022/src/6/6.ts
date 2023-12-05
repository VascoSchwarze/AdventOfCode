import fs from "fs";
const fileContent = fs.readFileSync("src/6/input.txt").toString();

for (let i = 3; i < fileContent.length; i++) {
	if (new Set([...fileContent.slice(i - 3, i + 1)]).size === 4) {
		console.log("Puzzle 1: ", i + 1);
        break;
	}
}

for (let i = 13; i < fileContent.length; i++) {
	if (new Set([...fileContent.slice(i - 13, i + 1)]).size === 14) {
		console.log("Puzzle 2: ", i + 1);
		break;
	}
}
