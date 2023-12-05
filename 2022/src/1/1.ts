import fs from "fs"
const fileContent = fs.readFileSync("src/1/input.txt").toString();

const elves: number[] = [0]
let curElfIdx = 0;

for (const line of fileContent.split("\n")) {
    if (line === "") {
        curElfIdx++; 
        elves[curElfIdx] = 0;
        continue;
    }

    elves[curElfIdx] += Number(line);
}

const maxCalories = Math.max(...elves);
console.log("Puzzle 1:", maxCalories);

const elvesSorted = elves.sort().reverse();
const top3Calories = elvesSorted[0] + elvesSorted[1] + elvesSorted[2];
console.log("Puzzle 2: ", top3Calories);