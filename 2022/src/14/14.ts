import fs from "fs";
const fileContent = fs.readFileSync("src/14/input.txt").toString();

let highestY = 0;
const blockedAreas: [Coordinates, Coordinates][] = [];

for (const line of fileContent.split("\n")) {
	const points = line.split(" -> ");
	let [firstX, firstY] = points[0].split(",");
	for (let nextIdx = 1; nextIdx < points.length; nextIdx++) {
		const nextPoint = points[nextIdx];
		const [nextX, nextY] = nextPoint.split(",");

		blockedAreas.push([
			[Number(firstX), Number(firstY)],
			[Number(nextX), Number(nextY)],
		]);

		if (Number(nextY) > highestY) highestY = Number(nextY);

		[firstX, firstY] = [nextX, nextY];
	}
}

const sandSource = [500, 0];
let restingSand: Coordinates[] = [];

const floor = highestY + 2;
blockedAreas.push([[sandSource[0] - floor, floor], [sandSource[0] + floor, floor]]);

const isPositionBlocked = (pos: Coordinates) => {
	for (const area of blockedAreas) {
		if (
			pos[0] <= Math.max(area[0][0], area[1][0]) &&
			pos[0] >= Math.min(area[0][0], area[1][0]) &&
			pos[1] <= Math.max(area[0][1], area[1][1]) &&
			pos[1] >= Math.min(area[0][1], area[1][1])
		)
			return true;
	}
	for (const sandPos of restingSand) {
		if (pos[0] === sandPos[0] && pos[1] === sandPos[1]) return true;
	}
	return false;
};

const getPossibleTargetPositionFrom = (initialPos: Coordinates): Coordinates | false => {
	if (!isPositionBlocked([initialPos[0], initialPos[1] + 1])) return [initialPos[0], initialPos[1] + 1];
	if (!isPositionBlocked([initialPos[0] - 1, initialPos[1] + 1]))
		return [initialPos[0] - 1, initialPos[1] + 1];
	if (!isPositionBlocked([initialPos[0] + 1, initialPos[1] + 1]))
		return [initialPos[0] + 1, initialPos[1] + 1];
	return false;
};

let [curSandX, curSandY] = [...sandSource];

while (curSandY <= highestY) {   //Puzzle 1
	let nextPos = getPossibleTargetPositionFrom([curSandX, curSandY]);
	if (nextPos === false) {
		restingSand.push([curSandX, curSandY]);
		[curSandX, curSandY] = [...sandSource];
		continue;
	}
	[curSandX, curSandY] = nextPos;
}

console.log("Puzzle 1: ", restingSand.length);


// -----------------  Puzzle 2  -----------------
// - brute force solution, takes a couple minutes to run, but works
// - the intended solution is a different one
// - one possibility might be to calculate the area of the triangle of sand that forms 
//   and subtract (1) the blocked areas and (2) the areas below those where sand can never get to


restingSand = [];
while (true) {
	let nextPos = getPossibleTargetPositionFrom([curSandX, curSandY]);
	if (nextPos === false) {
		restingSand.push([curSandX, curSandY]);
        // console.log(restingSand.length);
        if (curSandX === sandSource[0] && curSandY === sandSource[1]) break;
		[curSandX, curSandY] = [...sandSource];
		continue;
	}
	[curSandX, curSandY] = nextPos;
}

console.log("Puzzle 2: ", restingSand.length);

type Coordinates = [number, number];
