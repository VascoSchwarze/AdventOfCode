def task1(paths):
    path_count = 0
    for n in paths["start"]:
        path_count += visit(paths, n, ["start", n])

    return path_count


def task2(paths):
    path_count = 0
    for n in paths["start"]:
        path_count += visit2(paths, n, ["start", n])

    return path_count
    

def visit2(paths, node, cur_path):
    if node == "end":
        return 1
    path_count = 0
    for n in paths[node]:
        if n.isupper() or n == "end":
            path_count += visit2(paths, n, cur_path + [n])
            continue

        if n.islower():
            n_count = 0
            lowers = []
            some_twice = False
            for prev in [p for p in cur_path if p.islower()]:
                if prev == n:
                    n_count += 1
                if prev in lowers:
                    some_twice = True
                lowers += [prev]

            if n_count == 0 or (n_count == 1 and not some_twice):
                path_count += visit2(paths, n, cur_path + [n])

    return path_count


def visit(paths, node, cur_path):
    if node == "end":
        return 1
    path_count = 0
    for n in paths[node]:
        if n.isupper() or (n.islower() and n not in cur_path):
            path_count += visit(paths, n, cur_path + [n])

    return path_count



from collections import defaultdict

with open("Input") as f:
    content = [line.strip() for line in f.readlines()]

paths = defaultdict(list)
for line in content:
    a,b = line.split("-")
    if a == "start" or b == "end":
        paths[a] = paths[a] + [b]
        continue
    if b == "start" or a == "end":
        paths[b] = paths[b] + [a]
        continue
    paths[a] = paths[a] + [b]
    paths[b] = paths[b] + [a]

print(paths)

print("Task 1:", task1(paths))
print("Task 2:", task2(paths))