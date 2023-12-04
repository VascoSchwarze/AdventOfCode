class Node:
    def __init__(self, prev=None, position=None):
        self.prev = prev
        self.position = position
        
        self.g = 0
        self.h = 0
        self.f = 0

    def __eq__(self, other):
        return self.position == other.position
        

def task1(risks, grid_length):
    max_pos = len(risks) - 1
    start = (0,0)
    end = (max_pos, max_pos)
    return astar(start, end, risks, grid_length)



def task2(risks):
    max_pos = len(risks)
    new_risks = [row[:] for row in risks]
    for i in range(1,5):
        for y in range(max_pos):
            for x in range(max_pos):
                new_risks[y].append(risks[y][x] + i - 9 if risks[y][x] + i > 9 else risks[y][x] + i)

    for i in range(1,5):
        for y in range(max_pos):
            new_risks.append(new_risks[y][:])
            for x in range(len(new_risks[0])):
                new_risks[i * max_pos + y][x] = new_risks[i * max_pos + y][x] + i - 9 if new_risks[i * max_pos + y][x] + i > 9 else new_risks[i * max_pos + y][x] + i

    return task1(new_risks, len(risks))
    

def get_neighbors(grid, x_point, y_point):
    x_max = len(grid[0])
    y_max = len(grid)
    neighbors = [[-1,0], [1, 0], [0, -1], [0, 1]]
    ret = []
    for x_diff, y_diff in neighbors:
        x = x_point + x_diff
        y = y_point + y_diff
        if x >= 0 and x < x_max and y >= 0 and y < y_max:
            ret.append((x, y))

    return ret


def astar (p1, p2, cost, grid_length):
    #create start and end node
    start_node = Node(None, p1)
    start_node.g = start_node.h = start_node.f = 0
    end_node = Node(None, p2)
    
    #initialize open and closed list
    open_list = []
    closed_list = []
    
    #add start node
    open_list.append(start_node)
    
    #loop until end is found
    while len(open_list) > 0:
        l = len(closed_list)
        if l % 1000 == 0:
            print(l)

        #get current node
        # current_node = open_list[0]
        # current_index = 0
        # for index, item in enumerate(open_list):
        #     if item.f < current_node.f:
        #         current_node = item
        #         current_index = index

        open_list.sort(key=lambda node:node.f)
        
        current_node = open_list.pop(0)

        if current_node in closed_list:
            print("Hä")
        closed_list.append(current_node)
        
        #if the goal node is found, return the total cost
        if current_node == end_node:
            print(l)
            return current_node.g
        
        #generate neighbors
        neighbors = get_neighbors(cost, current_node.position[0], current_node.position[1])            
        
        #loop through neighbors
        for neighbor_pos in neighbors:
            neighbor = Node(current_node, neighbor_pos)
            
            closed = False
            for closed_node in closed_list:
                if neighbor == closed_node:
                    closed = True
                    break
            if closed:
                continue
                
            neighbor.g = current_node.g + cost[neighbor_pos[1]][neighbor_pos[0]]

            heuristic = 0
            for i in range(1,int(abs(end_node.position[1] - neighbor_pos[1]) / grid_length) + 1):
                # print(i)
                heuristic += i * grid_length

            i = int(abs(end_node.position[1] - neighbor_pos[1]) / grid_length) + 1
            heuristic += (abs(end_node.position[1] - neighbor_pos[1]) % grid_length) * i


            for i in range(1,int(abs(end_node.position[0] - neighbor_pos[0]) / grid_length) + 1):
                # print(i)
                heuristic += i * grid_length

            i = int(abs(end_node.position[0] - neighbor_pos[0]) / grid_length) + 1
            # print(abs(end_node.position[0] - neighbor_pos[0]))
            # print(grid_length)
            # print("i",i)
            # time.sleep(10)
            heuristic += (abs(end_node.position[0] - neighbor_pos[0]) % grid_length) * i

            neighbor.h = heuristic
            neighbor.f = neighbor.g + neighbor.h
            
            opened = False
            for open_node in open_list:
                if neighbor == open_node:
                    if neighbor.g >= open_node.g:
                        opened = True
                        break
                    else:
                        open_list.remove(open_node)
            if opened:
                continue
                
            open_list.append(neighbor)




from queue import PriorityQueue
import time

with open("Input") as f:
    content = [line.strip() for line in f.readlines()]

risks = [[int(c) for c in line] for line in content]

print("Task 1:", task1(risks, len(risks)))
print("Task 2:", task2(risks))