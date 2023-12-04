def task1(steps):
    hor_pos = 0
    depth = 0
    for step in steps:
        direction, number = step.split()
        if direction == "forward":
            hor_pos += int(number)
            continue
        if direction == "up":
            depth -= int(number)
            continue
        if direction == "down":
            depth += int(number)

    return hor_pos * depth


def task2(steps):
    hor_pos = 0
    depth = 0
    aim = 0
    for step in steps:
        direction, number = step.split()
        if direction == "forward":
            hor_pos += int(number)
            depth += aim * int(number)
            continue
        if direction == "up":
            aim -= int(number)
            continue
        if direction == "down":
            aim += int(number)

    return hor_pos * depth


with open("Input") as f:
    content = [line.strip() for line in f.readlines()]

print("Task 1:", task1(content))
print("Task 2:", task2(content))