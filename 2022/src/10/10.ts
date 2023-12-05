import fs from "fs";
const fileContent = fs.readFileSync("src/10/input.txt").toString();

let cycle = 0;
let register = 1;

let signalSum = 0;

let image: string[] = ["", "", "", "", "", ""];

const checkCycleCalc = () => {
    if (cycle === 20 || cycle === 60 || cycle === 100 || cycle === 140 || cycle === 180 || cycle === 220) {
        signalSum += cycle * register;
    }
}

const draw = () => {
    const col = cycle % 40;
    const row = (cycle - col) / 40; 
    if (row > 5) return;
    if (col >= register - 1 && col <= register + 1) {
        image[row] += "#";
    }
    else {
        image[row] += ".";
    }
}

for (const line of fileContent.split("\n")) {
    if (line.startsWith("noop")) {
        draw();
        cycle++;
        checkCycleCalc();
    }
    else {
        const [, number] = line.split(" ");
        draw();
        cycle++;
        checkCycleCalc();
        
        draw();
        cycle++;
        checkCycleCalc();
        register += Number(number);
    }
}

console.log("Puzzle 1: ", signalSum);

console.log("Puzzle 2: ");
console.log(image);