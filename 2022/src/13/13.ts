import fs from "fs";
const fileContent = fs.readFileSync("src/13/input.txt").toString();

let currentPackets: Element[][] = [];
let totalCorrectOrder = 0;
let curIndex = 1;
for (const line of fileContent.split("\n")) {
	if (line.length === 0) {
		if (compareLists(currentPackets[0], currentPackets[1]) === -1) totalCorrectOrder += curIndex;
		curIndex++;
		currentPackets = [];
	} else {
		currentPackets.push(JSON.parse(line));
	}
}

console.log("Puzzle 1: ", totalCorrectOrder);

// -----------  Puzzle 2  -----------

const packetList: Element[][] = [];
for (const line of fileContent.split("\n")) {
	if (line.length > 0) packetList.push(JSON.parse(line));
}

//divider packets
const div1 = [[2]];
const div2 = [[6]];
packetList.push(div1);
packetList.push(div2);

packetList.sort((a, b) => compareLists(a, b));

const div1Idx = packetList.findIndex(v => v === div1) + 1;
const div2Idx = packetList.findIndex(v => v === div2) + 1;
const decoderKey = div1Idx * div2Idx;

console.log("Puzzle 2: ", decoderKey);



function compareLists(a: Element[], b: Element[]): -1 | 0 | 1 {
	const aLength = a.length;
	const bLength = b.length;
	const commonElementCount = aLength < bLength ? aLength : bLength;
	for (let i = 0; i < commonElementCount; i++) {
		const comparisonResult = compareElements(a[i], b[i]);
		if (comparisonResult !== 0) return comparisonResult;
	}
	return aLength < bLength ? -1 : aLength > bLength ? 1 : 0;
}

function compareElements(a: Element, b: Element) {
	if (typeof a === "number") {
		if (typeof b === "number") return a < b ? -1 : a > b ? 1 : 0;
		else return compareLists([a], b);
	} else {
		if (typeof b === "number") return compareLists(a, [b]);
		else return compareLists(a, b);
	}
}

type Element = number | Element[];
