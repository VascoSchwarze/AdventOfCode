import fs from "fs";
const fileContent = fs.readFileSync("src/15/input.txt").toString();

const targetRow = 2000000;
const beaconsInTargetRow: number[] = [];
const coveredPositionsInTargetRow: Set<number> = new Set();

const sensors: {coords: Coordinates, beaconDistance: number}[] = [];

for (const line of fileContent.split("\n")) {
	const [sensorStr, beaconStr] = line.split(":");

	const [, , sensorXStr, sensorYStr] = sensorStr.split(" ");
	const sensorX = Number(sensorXStr.split(",")[0].split("=")[1]);
	const sensorY = Number(sensorYStr.split("=")[1]);

	const [, , , , , beaconXStr, beaconYStr] = beaconStr.split(" ");
	const beaconX = Number(beaconXStr.split(",")[0].split("=")[1]);
	const beaconY = Number(beaconYStr.split("=")[1]);
	if (beaconY === targetRow) beaconsInTargetRow.push(beaconX);

	const sensorToBeaconDistance = Math.abs(sensorX - beaconX) + Math.abs(sensorY - beaconY);
    sensors.push({coords: [sensorX, sensorY], beaconDistance: sensorToBeaconDistance});

	const sensorToTargetRowDistance = Math.abs(sensorY - targetRow);

	for (let i = 0; i <= sensorToBeaconDistance - sensorToTargetRowDistance; i++) {
		coveredPositionsInTargetRow.add(sensorX + i);
		coveredPositionsInTargetRow.add(sensorX - i);
	}
}

for (const beaconX of beaconsInTargetRow) {
	coveredPositionsInTargetRow.delete(beaconX);
}

console.log("Puzzle 1: ", coveredPositionsInTargetRow.size);

// ------------  Puzzle 2  ------------

const searchAreaWidth = 4000000;
const sensorEdgePositions: Coordinates[] = [];
for (const sensor of sensors) {
    const [sensorX, sensorY] = sensor.coords;
    const beaconDistance = sensor.beaconDistance;
    for (let x = sensorX - beaconDistance - 1; x <= sensorX + beaconDistance + 1; x++) {
        const edgePosition1: Coordinates = [x, sensorY + Math.abs(sensorX - x) - beaconDistance - 1];
        if (edgePosition1[0] >= 0 && edgePosition1[0] <= searchAreaWidth && edgePosition1[1] >= 0 && edgePosition1[1] <= searchAreaWidth)
            sensorEdgePositions.push(edgePosition1);

        const edgePosition2: Coordinates = [x, sensorY - Math.abs(sensorX - x) + beaconDistance + 1];
            if (edgePosition2[0] >= 0 && edgePosition2[0] <= searchAreaWidth && edgePosition2[1] >= 0 && edgePosition2[1] <= searchAreaWidth)
                sensorEdgePositions.push(edgePosition2);
    }
}


const remainingPosition = sensorEdgePositions.filter((coords) => {
    for (const sensor of sensors) {
        if (distance(coords, sensor.coords) <= sensor.beaconDistance)
        return false;
    }
    return true;
})

const tuningFrequency = 4000000 * remainingPosition[0][0] + remainingPosition[0][1];
console.log("Puzzle 2: ", tuningFrequency);


function distance(a: Coordinates, b: Coordinates) {
    return Math.abs(a[0]- b[0]) + Math.abs(a[1] - b[1]);
}

type Coordinates = [number, number];
