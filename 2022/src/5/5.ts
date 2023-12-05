import fs from "fs";
const fileContent = fs.readFileSync("src/5/input.txt").toString();

let stacksParsed = false;
const stacks: string[][] = [];
const instructions: Instruction[] = [];

lineParsingLoop: for (const line of fileContent.split("\n")) {
	if (line.startsWith("move")) {
		const [, count, , source, , target] = line.split(" ");
		instructions.push({ count: Number(count), source: Number(source) - 1, target: Number(target) - 1 });
	} else {
		if (stacksParsed) continue;
		for (let i = 0; i * 4 + 1 < line.length; i++) {
			const char = line.charAt(i * 4 + 1);
			if (char === "1") {
				//last line of crate stacks reached
				stacksParsed = true;
				continue lineParsingLoop;
			}
			if (char !== " ") {
				if (stacks[i] === undefined) stacks[i] = [];
				stacks[i].unshift(char);
			}
		}
	}
}

const stacks2: string[][] = JSON.parse(JSON.stringify(stacks));

for (const {count, source, target} of instructions) {
    for (let i = 0; i < count; i++) {
        const crate = stacks[source].pop();
        stacks[target].push(crate);
    }
}

let topCrates = "";
for (const stack of stacks) {
    topCrates += stack[stack.length - 1];
}

console.log("Puzzle 1: ", topCrates);


// -----------  Puzzle 2  -----------

for (const {count, source, target} of instructions) {
        const crates = stacks2[source].splice(stacks2[source].length - count, count);
        stacks2[target].push(...crates);
}

let topCrates2 = "";
for (const stack of stacks2) {
    topCrates2 += stack[stack.length - 1];
}

console.log("Puzzle 2: ", topCrates2);


type Instruction = {
	count: number;
	source: number;
	target: number;
};
