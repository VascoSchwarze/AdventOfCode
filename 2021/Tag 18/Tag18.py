def task1():
    pass


def task2():
    pass
    

def magnitude(number):
    match = re.match("\[(.*)\]", number)
    if "[" in match.group(1):
        pass
    else:
        pass


import re

with open("Input") as f:
    content = [line.strip() for line in f.readlines()]

print(magnitude())

# print("Task 1:", task1())
# print("Task 2:", task2())