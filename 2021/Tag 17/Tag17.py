def task1(bounds):
    y_vel = abs(bounds["y"][0] + 1)
    return y_vel * (y_vel + 1) / 2


def task2(bounds):
    y_pos = []
    for i in range(bounds["y"][0], abs(bounds["y"][0])):
        y_pos.append(i)

    x_pos = []
    for i in range(1, bounds["x"][1] + 1):
        if i * (i+1) / 2 >= bounds["x"][0]:
            x_pos.append(i)


    y_steps = {}
    max_y_steps = 0
    for y in y_pos:
        steps = []
        cur_steps = 0
        cur_y = 0
        y_vel = y
        while cur_y >= bounds["y"][0]:
            cur_y += y_vel
            y_vel -= 1
            cur_steps += 1
            if cur_y <= bounds["y"][1] and cur_y >= bounds["y"][0]:
                steps.append(cur_steps)
                if cur_steps > max_y_steps:
                    max_y_steps = cur_steps

        y_steps[y] = steps
    

    x_steps = {}
    for x in x_pos:
        steps = []
        cur_steps = 0
        cur_x = 0
        x_vel = x
        while cur_x <= bounds["x"][1] and x_vel != 0:
            cur_x += x_vel
            x_vel -= 1
            cur_steps += 1
            if cur_x <= bounds["x"][1] and cur_x >= bounds["x"][0]:
                steps.append(cur_steps)
                if x_vel == 0:
                    for i in range(cur_steps + 1, max_y_steps + 1):
                        steps.append(i)

        x_steps[x] = steps

    vel_count = 0
    vels = set()
    for y, y_step_counts in y_steps.items():
        for y_step_count in y_step_counts:
            for x, x_step_counts in x_steps.items():
                for x_step_count in x_step_counts:
                    if x_step_count == y_step_count:
                        vel_count += 1
                        vels.add((x, y))
                        break


    # print(len(vels))
    return len(vels)
        



# bounds = {
#     "x": [20, 30],
#     "y": [-10, -5]
# }


bounds = {
    "x": [135, 155],
    "y": [-102, -78]
}

s = set()

s.add((1,2))
s.add((1,2))
s.add((1,2))
print(s)

print("Task 1:", task1(bounds))
print("Task 2:", task2(bounds))