import fs from "fs";
const fileContent = fs.readFileSync("src/9/input.txt").toString();

// -----------  Puzzle 1  -----------
let headPosition: [number, number] = [0, 0];
let tailPosition: [number, number] = [0, 0];
let visitedPositions: Set<string> = new Set(["0,0"]);
for (const line of fileContent.split("\n")) {
	const [direction, stepsStr] = line.split(" ");
	const steps = Number(stepsStr);
	for (let i = 0; i < steps; i++) {
		switch (direction) {
			case "R":
				headPosition[0]++;
				break;
			case "L":
				headPosition[0]--;
				break;
			case "U":
				headPosition[1]++;
				break;
			case "D":
				headPosition[1]--;
				break;
		}

		tailPosition = updatePosition(headPosition, tailPosition);

		visitedPositions.add(tailPosition[0] + "," + tailPosition[1]);
	}
}

console.log("Puzzle 1: ", visitedPositions.size);

// -----------  Puzzle 2  -----------

headPosition = [0, 0];
let tailPositions: [number, number][] = [];
for (let i = 0; i < 9; i++) tailPositions[i] = [0, 0];

visitedPositions = new Set(["0,0"]);
for (const line of fileContent.split("\n")) {
	const [direction, stepsStr] = line.split(" ");
	const steps = Number(stepsStr);
	for (let i = 0; i < steps; i++) {
		switch (direction) {
			case "R":
				headPosition[0]++;
				break;
			case "L":
				headPosition[0]--;
				break;
			case "U":
				headPosition[1]++;
				break;
			case "D":
				headPosition[1]--;
				break;
		}

        tailPositions[0] = updatePosition(headPosition, tailPositions[0]);

		for (let i = 1; i < 9; i++) {
            tailPositions[i] = updatePosition(tailPositions[i - 1], tailPositions[i]);
        }

        const lastTailPos = tailPositions[8];

		visitedPositions.add(lastTailPos[0] + "," + lastTailPos[1]);
	}
}

console.log("Puzzle 2: ", visitedPositions.size);

function updatePosition(
	firstPosition: [number, number],
	followingPosition: [number, number]
): [number, number] {
	const dx = firstPosition[0] - followingPosition[0];
	const dy = firstPosition[1] - followingPosition[1];
	const newPosition: [number, number] = [...followingPosition];
	const touching = Math.max(Math.abs(dx), Math.abs(dy)) > 1 ? false : true;
	if (!touching) {
		if (dx > 1 && dy == 0) newPosition[0]++;
		else if (dx < -1 && dy == 0) newPosition[0]--;
		else if (dy > 1 && dx == 0) newPosition[1]++;
		else if (dy < -1 && dx == 0) newPosition[1]--;
		else if ((dx > 1 && dy > 0) || (dy > 1 && dx > 0)) {
			newPosition[0]++;
			newPosition[1]++;
		} else if ((dx > 1 && dy < 0) || (dy < -1 && dx > 0)) {
			newPosition[0]++;
			newPosition[1]--;
		} else if ((dx < -1 && dy > 0) || (dy > 1 && dx < 0)) {
			newPosition[0]--;
			newPosition[1]++;
		} else if ((dx < -1 && dy < 0) || (dy < -1 && dx < 0)) {
			newPosition[0]--;
			newPosition[1]--;
		}
	}
	return newPosition;
}
