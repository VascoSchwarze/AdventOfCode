def task1(levels):
    flash_count = 0
    x_max = len(levels[0])
    y_max = len(levels)
    for i in range(100):
        for x in range(x_max):
            for y in range(y_max):
                levels[y][x] += 1

        flashed = True
        while flashed:
            flashed = False
            for x in range(x_max):
                for y in range(y_max):
                    if levels[y][x] > 9:
                        flash(levels, x, y)
                        flashed = True
                        flash_count += 1
                        levels[y][x] = 0

    return flash_count


def task2(levels):
    x_max = len(levels[0])
    y_max = len(levels)
    flash_count = 0
    step = 0
    while flash_count != 100:
        step += 1
        flash_count = 0
        for x in range(x_max):
            for y in range(y_max):
                levels[y][x] += 1

        flashed = True
        while flashed:
            flashed = False
            for x in range(x_max):
                for y in range(y_max):
                    if levels[y][x] > 9:
                        flash(levels, x, y)
                        flashed = True
                        flash_count += 1
                        levels[y][x] = 0

    return step
    

def flash(levels, x_point, y_point):
    x_max = len(levels[0])
    y_max = len(levels)
    neighbors = [[-1,0], [1, 0], [0, -1], [0, 1], [-1, -1], [-1, 1], [1, -1], [1, 1]]
    for x_diff, y_diff in neighbors:
        x = x_point + x_diff
        y = y_point + y_diff
        if x >= 0 and x < x_max and y >= 0 and y < y_max:
            if levels[y][x] != 0:
                levels[y][x] += 1

    
         

with open("Input") as f:
    content = [line.strip() for line in f.readlines()]

levels = [[int(c) for c in line] for line in content]

print("Task 1:", task1([row[:] for row in levels]))
print("Task 2:", task2(levels))