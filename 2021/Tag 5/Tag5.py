def task1(lines):
    points = {}
    for line in lines:
        (x1, y1) = line[0]
        (x2, y2) = line[1]
        if x1 == x2:
            if y1 > y2:
                for y in range(y2, y1+1):
                    points[(x1,y)] = points.get((x1,y),0) + 1
            else: 
                for y in range(y1, y2+1):
                    points[(x1,y)] = points.get((x1,y),0) + 1
        
        if y1 == y2:
            if x1 > x2:
                for x in range(x2, x1+1):
                    points[(x,y1)] = points.get((x,y1),0) + 1
            else: 
                for x in range(x1, x2+1):
                    points[(x,y1)] = points.get((x,y1),0) + 1

    count = 0
    for key in points.keys():
        if points[key] > 1:
            count += 1

    return count


def task2(lines):
    points = {}
    for line in lines:
        (x1, y1) = line[0]
        (x2, y2) = line[1]
        if x1 == x2:
            if y1 > y2:
                for y in range(y2, y1+1):
                    points[(x1,y)] = points.get((x1,y), 0) + 1
            else: 
                for y in range(y1, y2+1):
                    points[(x1,y)] = points.get((x1,y), 0) + 1
        elif y1 == y2:
            if x1 > x2:
                for x in range(x2, x1+1):
                    points[(x,y1)] = points.get((x,y1), 0) + 1
            else: 
                for x in range(x1, x2+1):
                    points[(x,y1)] = points.get((x,y1), 0) + 1
        else:
            if x1 > x2:
                for x in range(x2, x1+1):
                    if y1 > y2:
                        points[(x, y2+x-x2)] = points.get((x, y2+x-x2), 0) + 1
                    else:
                        points[(x, y2-x+x2)] = points.get((x, y2-x+x2), 0) + 1
            else:
                for x in range(x1, x2+1):
                    if y1 > y2:
                        points[(x, y1-x+x1)] = points.get((x, y1-x+x1), 0) + 1
                    else:
                        points[(x, y1+x-x1)] = points.get((x, y1+x-x1), 0) + 1


    count = 0
    for key in points.keys():
        if points[key] > 1:
            count += 1

    return count


with open("Input") as f:
    content = [line.strip() for line in f.readlines()]

lines = []
for line in content:
    coords = line.split(" -> ")
    x1, y1 = coords[0].split(",")
    x2, y2 = coords[1].split(",")
    lines.append([(int(x1),int(y1)), (int(x2),int(y2))])


print("Task 1:", task1(lines))
print("Task 2:", task2(lines))