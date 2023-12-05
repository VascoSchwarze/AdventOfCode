const monkeys: Monkey[] = [
    {
        items: [63, 84, 80, 83, 84, 53, 88, 72],
        operation: (old) => old * 11,
        test: (worryLevel) => worryLevel % 13 === 0 ? 4 : 7
    },
    {
        items: [67, 56, 92, 88, 84],
        operation: (old) => old + 4,
        test: (worryLevel) => worryLevel % 11 === 0 ? 5 : 3
    },
    {
        items: [52],
        operation: (old) => old * old,
        test: (worryLevel) => worryLevel % 2 === 0 ? 3 : 1
    },
    {
        items: [59, 53, 60, 92, 69, 72],
        operation: (old) => old + 2,
        test: (worryLevel) => worryLevel % 5 === 0 ? 5 : 6
    },
    {
        items: [61, 52, 55, 61],
        operation: (old) => old + 3,
        test: (worryLevel) => worryLevel % 7 === 0 ? 7 : 2
    },
    {
        items: [79, 53],
        operation: (old) => old + 1,
        test: (worryLevel) => worryLevel % 3 === 0 ? 0 : 6
    },
    {
        items: [59, 86, 67, 95, 92, 77, 91],
        operation: (old) => old + 5,
        test: (worryLevel) => worryLevel % 19 === 0 ? 4 : 0
    },
    {
        items: [58, 83, 89],
        operation: (old) => old * 19,
        test: (worryLevel) => worryLevel % 17 === 0 ? 2 : 1
    }
]

const monkeys2: Monkey[] = monkeys.map(m => {return {...m, items: [...m.items]}});

let totalInspectCount = [0, 0, 0, 0, 0, 0, 0, 0];
for (let i = 0; i < 20; i++) {
    for (let m = 0; m < monkeys.length; m++) {
        const monkey = monkeys[m];

        for (let itemWorryLevel of monkey.items) {
            itemWorryLevel = monkey.operation(itemWorryLevel);
            totalInspectCount[m]++;
            itemWorryLevel = Math.floor(itemWorryLevel / 3);
            const targetMonkey = monkey.test(itemWorryLevel);
            monkeys[targetMonkey].items.push(itemWorryLevel);
        }
        monkey.items = [];
    }
}

const inspectCountSorted = totalInspectCount.sort((a,b) => a<b?1:-1);
const monkeyBusiness = inspectCountSorted[0] * inspectCountSorted[1];


console.log("Puzzle 1: ", monkeyBusiness);

// -------------  Puzzle 2  -------------

let totalInspectCount2 = [0, 0, 0, 0, 0, 0, 0, 0];
for (let i = 0; i < 10000; i++) {
    for (let m = 0; m < monkeys2.length; m++) {
        const monkey = monkeys2[m];
        for (let itemWorryLevel of monkey.items) {
            itemWorryLevel = monkey.operation(itemWorryLevel);
            totalInspectCount2[m]++;
            itemWorryLevel = itemWorryLevel % (2 * 3 * 5 * 7 * 11 * 13 * 17 * 19); // division rest stays the same (for all monkeys) but the number gets smaller
            const targetMonkey = monkey.test(itemWorryLevel);
            monkeys2[targetMonkey].items.push(itemWorryLevel);
        }
        monkey.items = [];
    }
}

const inspectCountSorted2 = totalInspectCount2.sort((a,b) => a<b?1:-1);
const monkeyBusiness2 = inspectCountSorted2[0] * inspectCountSorted2[1];


console.log("Puzzle 2 ", monkeyBusiness2);




type Monkey = {
    items: number[];
    operation: (old: number) => number;
    test: (worryLevel: number) => number;
}



// const monkeys: Monkey[] = [
//     {
//         items: [79, 98],
//         operation: (old) => old * 19,
//         test: (worryLevel) => worryLevel % 23 === 0 ? 2 : 3
//     },
//     {
//         items: [54, 65, 75, 74],
//         operation: (old) => old + 6,
//         test: (worryLevel) => worryLevel % 19 === 0 ? 2 : 0
//     },
//     {
//         items: [79, 60, 97],
//         operation: (old) => old * old,
//         test: (worryLevel) => worryLevel % 13 === 0 ? 1 : 3
//     },
//     {
//         items: [74],
//         operation: (old) => old + 3,
//         test: (worryLevel) => worryLevel % 17 === 0 ? 0 : 1
//     }
// ]