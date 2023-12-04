def task1(content):
    score = {
        ')': 3,
        ']': 57,
        '}': 1197,
        '>': 25137
    }  
    bracket = {
        '(': ')',
        '[': ']',
        '{': '}',
        '<': '>'
    }
    sum = 0
    for line in content:
        stack = deque()
        for char in line:
            if char == '(' or char == '{' or char == '[' or char == '<':
                stack.append(char)
                continue

            if char != bracket[stack.pop()]:
                sum += score[char]
                break
                
    return sum


def task2(content):
    score = {
        ')': 1,
        ']': 2,
        '}': 3,
        '>': 4
    }  
    bracket = {
        '(': ')',
        '[': ']',
        '{': '}',
        '<': '>'
    }
    point_list = []
    for line in content:
        stack = deque()
        corrupt = False
        for char in line:
            if char == '(' or char == '{' or char == '[' or char == '<':
                stack.append(bracket[char])
                continue

            if char != stack.pop():
                corrupt = True
                break

        if corrupt:
            continue
        else:
            points = 0
            count = len(stack)
            for i in range(count):
                points *= 5
                points += score[stack.pop()]
            point_list.append(points)
                
    point_list.sort()
    return point_list[int((len(point_list) - 1) / 2)]
    

from collections import deque

with open("Input") as f:
    content = [line.strip() for line in f.readlines()]

print("Task 1:", task1(content))
print("Task 2:", task2(content))