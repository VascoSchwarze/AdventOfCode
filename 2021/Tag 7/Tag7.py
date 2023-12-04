def task1(crabs):
    mi = min(crabs)
    ma = max(crabs)
    best = len(crabs) * ma

    for i in range(mi, ma+1):
        sum = 0
        for crab in crabs:
            sum += abs(i - crab)
            
        best = sum if sum < best else best

    return best




def task2(crabs):
    mi = min(crabs)
    ma = max(crabs)
    best = len(crabs) * ma * (ma + 1)

    for i in range(mi, ma+1):
        sum = 0
        for crab in crabs:
            n = abs(i - crab)
            sum += int(n * (n + 1) / 2)
            
        best = sum if sum < best else best

    return best
    


with open("Input") as f:
    content = [line.strip() for line in f.readlines()]

crabs = [int(crab) for crab in content[0].split(",")]

print("Task 1:", task1(crabs[:]))
print("Task 2:", task2(crabs[:]))