import fs from "fs"
const fileContent = fs.readFileSync("src/4/input.txt").toString();

let pairsContaining = 0;
let pairsOverlapping = 0;
for (const line of fileContent.split("\n")) {
    const [firstArea, secondArea] = line.split(",");
    const [start1, end1] = firstArea.split("-").map(str => Number(str));
    const [start2, end2] = secondArea.split("-").map(str => Number(str));

    const firstContainedInSecond = start1 >= start2 && end1 <= end2;
    const secondContainedInFirst = start2 >= start1 && end2 <= end1;
    if (firstContainedInSecond || secondContainedInFirst) pairsContaining++;


    const isStart1InArea2 = start1 >= start2 && start1 <= end2;
    const isEnd1InArea2 = end1 >= start2 && end1 <= end2;
    // only case with overlapping not yet covered is area1 completely containing area2 which is being checked here
    const isStart2InArea1 = start2 >= start1 && start2 <= end1;
    if (isStart1InArea2 || isEnd1InArea2 || isStart2InArea1) pairsOverlapping++;
}

console.log("Puzzle 1: ", pairsContaining);

console.log("Puzzle 2: ", pairsOverlapping);