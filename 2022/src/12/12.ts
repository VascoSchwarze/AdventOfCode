import fs from "fs";
const fileContent = fs.readFileSync("src/12/input.txt").toString();

const grid: Node[][] = [];
let start: Node;
let end: Node;

const lines = fileContent.split("\n");
for (let row = 0; row < lines.length; row++) {
	const nodes: Node[] = [];
	const chars = lines[row].split("");
	for (let col = 0; col < chars.length; col++) {
		const c = chars[col];
		let node: Node;
		if (c === "S") {
			node = {
				height: 0,
				row,
				col,
				distance: 0,
				visited: false,
			};
			start = node;
		} else if (c === "E") {
			node = {
				height: 25,
				row,
				col,
				distance: Number.MAX_SAFE_INTEGER,
				visited: false,
			};
			end = node;
		} else
			node = {
				height: c.charCodeAt(0) - 97,
				row,
				col,
				distance: Number.MAX_SAFE_INTEGER,
				visited: false,
			};
		nodes.push(node);
	}
	grid.push(nodes);
}

const queue: Node[] = [start];

while (queue.length > 0) {
	// console.log(queue);
	let currentNode = queue.sort((a, b) => (a.distance < b.distance ? -1 : 1)).shift();
	const neighbors: Node[] = [];
	for (const [x, y] of [
		[-1, 0],
		[1, 0],
		[0, -1],
		[0, 1],
	]) {
		const newY = currentNode.row + y;
		const newX = currentNode.col + x;
		if (newY >= 0 && newY < grid.length && newX >= 0 && newX < grid[0].length) {
			const potentialNeighbor = grid[currentNode.row + y][currentNode.col + x];
			if (potentialNeighbor.height <= currentNode.height + 1) neighbors.push(potentialNeighbor);
		}
	}

	for (const neighbor of neighbors) {
		if (!neighbor.visited) {
			const newDist = currentNode.distance + 1;
			if (newDist < neighbor.distance) neighbor.distance = newDist;

			if (!queue.includes(neighbor)) queue.push(neighbor);
		}
	}
	currentNode.visited = true;
}

const shortestPath = grid[end.row][end.col].distance;
console.log("Puzzle 1: ", shortestPath);

// Puzzle 2 solved by looking at the input: the shortest path from any "a" starts at the "a" node 6 rows lower, so its distance is 6 shorter

type Node = {
	height: number;
	row: number;
	col: number;
	distance: number;
	visited: boolean;
};
