import fs from "fs";
const fileContent = fs.readFileSync("src/8/input.txt").toString();

const trees: Tree[][] = [];
for (const line of fileContent.split("\n")) {
	const treeRow: Tree[] = [];
	for (const c of line) treeRow.push({ height: Number(c), visible: false });
	trees.push(treeRow);
}

for (const row of trees) {
	let largestTreeSeen = -1;
	for (let i = 0; i < row.length; i++) {
		if (row[i].height > largestTreeSeen) {
			row[i].visible = true;
			largestTreeSeen = row[i].height;
		}
	}

	largestTreeSeen = -1;
	for (let i = row.length - 1; i > -1; i--) {
		if (row[i].height > largestTreeSeen) {
			row[i].visible = true;
			largestTreeSeen = row[i].height;
		}
	}
}

for (let i = 0; i < trees[0].length; i++) {
	let largestTreeSeen = -1;
	const col = getColumn(trees, i);
	for (let j = 0; j < col.length; j++) {
		if (col[j].height > largestTreeSeen) {
			col[j].visible = true;
			largestTreeSeen = col[j].height;
		}
	}

	largestTreeSeen = -1;
	for (let j = col.length - 1; j > -1; j--) {
		if (col[j].height > largestTreeSeen) {
			col[j].visible = true;
			largestTreeSeen = col[j].height;
		}
	}
}

const treesVisible = trees.reduce((prev, cur) => {
	return prev + cur.reduce((prev, cur) => prev + (cur.visible ? 1 : 0), 0);
}, 0);

console.log("Puzzle 1: ", treesVisible);


// --------------  Puzzle 2  --------------

// we can skip the trees on the edges because they all have a scenic score of 0
let maxScenicSore = -1;
for (let rowIdx = 1; rowIdx < trees.length - 1; rowIdx++) {
	const row = trees[rowIdx];
	for (let colIdx = 1; colIdx < row.length - 1; colIdx++) {
		const col = getColumn(trees, colIdx);
		const curTree = row[colIdx];

		let leftViewDist = 0;
		do {
			leftViewDist++;
		} while (colIdx - leftViewDist > 0 && trees[rowIdx][colIdx - leftViewDist].height < curTree.height);

		let rightViewDist = 0;
		do {
			rightViewDist++;
		} while (
			colIdx + rightViewDist < row.length - 1 &&
			trees[rowIdx][colIdx + rightViewDist].height < curTree.height
		);

		let upViewDist = 0;
		do {
			upViewDist++;
		} while (rowIdx - upViewDist > 0 && trees[rowIdx - upViewDist][colIdx].height < curTree.height);

		let downViewDist = 0;
		do {
			downViewDist++;
		} while (
			rowIdx + downViewDist < col.length - 1 &&
			trees[rowIdx + downViewDist][colIdx].height < curTree.height
		);

		const curScenicSore = leftViewDist * rightViewDist * upViewDist * downViewDist;
		if (curScenicSore > maxScenicSore) maxScenicSore = curScenicSore;
	}
}

console.log("Puzzle 2: ", maxScenicSore);



function getColumn(array: Tree[][], col: number) {
	return array.map((row) => row[col]);
}

type Tree = {
	height: number;
	visible: boolean;
};
