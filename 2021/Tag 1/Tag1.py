def task1(depths):
    count = 0
    last_depth = depths[0]
    for i in range(1, len(depths)):
        if depths[i] > last_depth:
            count += 1

        last_depth = depths[i]

    return count


def task2(depths):
    count = 0
    last_window = depths[0] + depths[1] + depths[2]
    for i in range(3, len(depths)):
        new_window = last_window - depths[i-3] + depths[i]
        if new_window > last_window:
            count += 1

        last_window = new_window

    return count


with open("Input") as f:
    content = [int(line.strip()) for line in f.readlines()]

print("Task 1:", task1(content))
print("Task 2:", task2(content))