import fs from "fs"
const fileContent = fs.readFileSync("src/2/input.txt").toString();

let totalScoreV1 = 0, totalScoreV2 = 0;
for (const line of fileContent.split("\n")) {
    const [p1, response] = line.split(" ");
    totalScoreV1 += getGameScoreVariant1(p1, response);
    totalScoreV2 += getGameScoreVariant2(p1, response);
}

console.log("Puzzle 1:", totalScoreV1);
console.log("Puzzle 2:", totalScoreV2);

function getGameScoreVariant1(p1: string, response: string) {
    if (p1 === "A") {
        if (response === "X") return 4;
        if (response === "Y") return 8;
        if (response === "Z") return 3;
    }
    else if (p1 === "B") {
        if (response === "X") return 1;
        if (response === "Y") return 5;
        if (response === "Z") return 9;
    }
    else {
        if (response === "X") return 7;
        if (response === "Y") return 2;
        if (response === "Z") return 6;
    }
}

function getGameScoreVariant2(p1: string, gameResult: string) {
    if (p1 === "A") {
        if (gameResult === "X") return 3;
        if (gameResult === "Y") return 4;
        if (gameResult === "Z") return 8;
    }
    else if (p1 === "B") {
        if (gameResult === "X") return 1;
        if (gameResult === "Y") return 5;
        if (gameResult === "Z") return 9;
    }
    else {
        if (gameResult === "X") return 2;
        if (gameResult === "Y") return 6;
        if (gameResult === "Z") return 7;
    }
}