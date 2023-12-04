def task1(grid):
    sum = 0
    for y in range(len(grid)):
        row = grid[y]
        for x in range(len(row)):
            if is_low_point(grid, x, y):
                sum += 1 + grid[y][x]
            
    return sum

def task2(grid):
    basins = defaultdict(int)
    for y in range(len(grid)):
        row = grid[y]
        for x in range(len(row)):
            low_point = (x_low, y_low) = check_basin(grid, x, y)
            if x_low == -1:
                continue
            else:
                basins[low_point] += 1

    basins_values = list(basins.values())
    basins_values.sort(reverse=True)
    product = 1
    for i in range(3):
        product = product * basins_values[i]

    return product


def check_basin(grid, x_point, y_point):
    x_max = len(grid[0])
    y_max = len(grid)

    point = grid[y_point][x_point]
    if point == 9: 
        return -1, -1

    neighbors = [[-1,0], [1, 0], [0, -1], [0, 1]]
    for x_diff, y_diff in neighbors:
        x = x_point + x_diff
        y = y_point + y_diff
        if x >= 0 and x < x_max and y >= 0 and y < y_max:
            if grid[y][x] < point:
                return check_basin(grid, x, y)

    return x_point, y_point

def is_low_point(grid, x_point, y_point):
    x_max = len(grid[0])
    y_max = len(grid)
    point = grid[y_point][x_point]
    neighbors = [[-1,0], [1, 0], [0, -1], [0, 1]]
    higher_neighs = 0
    total_neighbours = 0
    for x_diff, y_diff in neighbors:
        x = x_point + x_diff
        y = y_point + y_diff
        if x >= 0 and x < x_max and y >= 0 and y < y_max:
            total_neighbours += 1
            if grid[y][x] > point:
                higher_neighs += 1

    if higher_neighs == total_neighbours:
        return True
    return False


from collections import defaultdict

with open("Input") as f:
    content = [line.strip() for line in f.readlines()]

grid = [[int(num) for num in line] for line in content]

print("Task 1:", task1(grid))
print("Task 2:", task2(grid))