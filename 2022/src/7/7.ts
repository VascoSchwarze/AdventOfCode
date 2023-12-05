import fs from "fs";
const fileContent = fs.readFileSync("src/7/input.txt").toString();

const root: Directory = {};
let currentPath: string[] = [];

for (const line of fileContent.split("\n")) {
    if (line.startsWith("$")){
        const [, cmd, arg] = line.split(" ");
        if (cmd === "cd") {
            if (arg === "/") currentPath = [];
            else if (arg === "..") currentPath.pop();
            else currentPath.push(arg);
        }
    }
    else if (line.startsWith("dir")) {
        const [, name] = line.split(" ");
        let curDir = root;
        for (const dir of currentPath) curDir = curDir[dir] as Directory;
        curDir[name] = {};
    }
    else {
        const [sizeStr, name] = line.split(" ");
        const size = Number(sizeStr);
        let curDir = root;
        for (const dir of currentPath) curDir = curDir[dir] as Directory;
        curDir[name] = {size};
    }
}

const smallDirs = findSmallDirsIn(root);

const smallDirSum = smallDirs.reduce((prev, cur) => {
    return prev += cur.size;
}, 0);

console.log("Puzzle 1: ", smallDirSum);

// --------------  Puzzle 2  --------------

const spaceToFree = 30_000_000 - (70_000_000 - getSizeOf(root));

const smallestDirSizeToFree = getSmallestDirOverThreshold(root, spaceToFree);

console.log("Puzzle 2: ", smallestDirSizeToFree);


function getSmallestDirOverThreshold (dir: Directory, threshold: number) {
    const dirSize = getSizeOf(dir);
    let smallestDirOverThreshold = dirSize > threshold ? dirSize : Number.MAX_SAFE_INTEGER;
    for (const entry of Object.values(dir)) {
        if (entry.size !== undefined) continue; // entry is a file

        const smallestChildDirSize = getSmallestDirOverThreshold(entry as Directory, threshold);
        if (smallestChildDirSize < smallestDirOverThreshold && smallestChildDirSize >= threshold) smallestDirOverThreshold = smallestChildDirSize;
    }
    return smallestDirOverThreshold;
}

function findSmallDirsIn (dir: Directory) : {name: string; size: number}[] {
    const dirs: {name: string; size: number}[] = [];
    for (const [name, entry] of Object.entries(dir)) {
        if (entry.size !== undefined) continue; // entry is a file

        const dirSize = getSizeOf(entry as Directory);
        if (dirSize <= 100000) dirs.push({name, size: dirSize});
        dirs.push(...findSmallDirsIn(entry as Directory));
    }

    return dirs;
}

function getSizeOf (dir: Directory) {
    let size = 0;
    for (const entry of Object.values(dir)) {
        if (entry.size !== undefined && typeof entry.size === "number") size += entry.size;
        else size += getSizeOf(entry as Directory);
    }
    return size;
}


type File = {
    size: number;
}

type Directory = {
    [key: string]: Directory | File;
}