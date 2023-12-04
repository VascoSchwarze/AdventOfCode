from collections import defaultdict


def task1(template, rules):
    cur_str = template
    for i in range(10):
        new_str = cur_str[0]
        for i in range(len(cur_str) - 1):
            pair = cur_str[i] + cur_str[i+1]
            if pair in rules:
                new_str += rules[pair]
            new_str += cur_str[i+1]

        cur_str = new_str


    counts = []
    for letter in set(cur_str):
        counts.append(cur_str.count(letter))

    counts.sort()
    return counts[-1] - counts[0]



def task2(template, rules):
    new_rules = {}
    for rule in rules:
        inserted = rule[0]+rules[rule]+rule[1]
        new_rules[rule] = [inserted[0] + inserted[1], inserted[1] + inserted[2]]

    pair_count = defaultdict(int)
    for i in range(len(template) - 1):
        pair = template[i] + template[i + 1]
        pair_count[pair] += 1


    for i in range(40):
        new_pair_count = defaultdict(int)
        for pair in pair_count:
            for new_pair in new_rules[pair]:
                new_pair_count[new_pair] += pair_count[pair]

        pair_count = new_pair_count


    letters = defaultdict(int)
    for pair in pair_count:
        for letter in pair:
            letters[letter] += pair_count[pair]

    for letter in letters:
        if letter == template[0] or letter == template[-1]:
            letters[letter] = (letters[letter] - 1) / 2 + 1
        else:
            letters[letter] /= 2

    counts = [letters[letter] for letter in letters]
    counts.sort()
    return counts[-1] - counts[0]
            
    

from collections import defaultdict

with open("Input") as f:
    content = [line.strip() for line in f.readlines()]

template = content[0]
rules = {}

for line in content[2:]:
    pair, elem = line.split(" -> ")
    rules[pair] = elem

print("Task 1:", task1(template, rules))
print("Task 2:", task2(template, rules))