def task1(fish):
    for day in range(80):
        for i in range(len(fish)):
            new_fish = 0
            if fish[i] == 0:
                fish[i] = 6
                new_fish += 1
            else:
                fish[i] -= 1
            
            for new in range(new_fish):
                fish.append(8)

    return len(fish)


def task2(fish):
    fish_count = len(fish)
    new_fish = {}
    for f in fish:
        new_fish[f] = new_fish.get(f, 0) + 1 

    for day in range(256):
        fish_count += new_fish.get(day,0)
        new_fish[day + 7] = new_fish.get(day + 7, 0) + new_fish.get(day, 0)
        new_fish[day + 9] = new_fish.get(day + 9, 0) + new_fish.get(day, 0)

    return fish_count
    


with open("Input") as f:
    content = [line.strip() for line in f.readlines()]

fish = [int(num) for num in content[0].split(",")]


print("Task 1:", task1(fish[:]))
print("Task 2:", task2(fish[:]))