def task1(points, fold):
    new_points = set()
    if fold[0] == "y":
        for point in points:
            x,y = point
            if y > fold[1]:
                new_points.add((x, 2 * fold[1] - y))
            else:
                new_points.add(point)
    else:
        for point in points:
            x,y = point
            if x > fold[1]:
                new_points.add((2 * fold[1] - x, y))
            else:
                new_points.add(point)

    return len(new_points)


def task2(points, folds):
    for fold in folds:
        new_points = set()
        if fold[0] == "y":
            for point in points:
                x,y = point
                if y > fold[1]:
                    new_points.add((x, 2 * fold[1] - y))
                else:
                    new_points.add(point)
        else:
            for point in points:
                x,y = point
                if x > fold[1]:
                    new_points.add((2 * fold[1] - x, y))
                else:
                    new_points.add(point)
    
        points = new_points

    x_values = [p[0] for p in points]
    y_values = [p[1] for p in points]
    x_max = max(x_values)
    y_max = max(y_values)
    for y in range(y_max + 1):
        line = ""
        for x in range(x_max + 1):
            if (x,y) in points:
                line += "⬜"
            else: 
                line += "⬛"
        
        print(line)


with open("Input") as f:
    content = [line.strip() for line in f.readlines()]

folds = []
points = []

for line in content:
    if "fold" in line:
        direction, number = line.split("along ")[1].split("=")
        folds.append((direction, int(number)))
    elif line == "":
        continue
    else:
        x,y = line.split(",")
        points.append((int(x), int(y)))

print("Task 1:", task1(points, folds[0]))
print("Task 2:", task2(points, folds))