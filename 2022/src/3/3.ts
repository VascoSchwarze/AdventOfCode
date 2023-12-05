import fs from "fs"
const fileContent = fs.readFileSync("src/3/input.txt").toString();

const wrongItems: string[] = [];
for (const line of fileContent.split("\n")) {
    const comp1 = line.substring(0, (line.length - 1) / 2 + 1);
    const comp2 = line.substring((line.length - 1) / 2 + 1, line.length);
    const wrongItem = new Set([...comp1].filter(item => comp2.includes(item)));
    if (wrongItem.size !== 1) {
        console.log("Wrong: ", wrongItem);
        console.log(line);
        console.log(comp1);
        console.log(comp2);
    }
    wrongItems.push(...wrongItem.values())
}

let prioSum = 0;
for (const item of wrongItems) {
    prioSum += getItemPriority(item);
}

console.log("Puzzle 1: ", prioSum);


// ---------------  Puzzle 2  ------------------

const lines = fileContent.split("\n");
let curItemCollection: number[] = [];
const badges: string[] = [];
for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    const itemsSet = new Set([...line]);
    for (const item of itemsSet) {
        const charCode = item.charCodeAt(0);
        if (curItemCollection[charCode] === undefined)
            curItemCollection[charCode] = 1;
        else
            curItemCollection[charCode] += 1;
    }

    if (i % 3 === 2) {
        const badgeCode = curItemCollection.indexOf(3);
        badges.push(String.fromCharCode(badgeCode));
        curItemCollection = [];
    }
}

let prioSum2 = 0;
for (const item of badges) {
    prioSum2 += getItemPriority(item);
}

console.log("Puzzle 2: ", prioSum2);



function getItemPriority (item: string) {
    const charCode = item.charCodeAt(0);
    if (charCode < 97) return charCode - 38;
    else return charCode - 96;
}