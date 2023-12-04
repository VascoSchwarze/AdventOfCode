class Board:
    def __init__(self, numbers):
        self.numbers = numbers
        self.marked = [[0,0,0,0,0] for i in range(5)]

    def mark(self, number):
        for i in range(5):
            row = self.numbers[i]
            for j in range(5):
                num = row[j]
                if num == number:
                    self.marked[i][j] = 1

    def check_complete(self):
        ret = False
        rows = self.marked
        cols = [[row[i] for row in self.marked] for i in range(5)]
        lines = rows + cols
        for line in lines:
            if len(set(line)) == 1 and line[0] == 1:
                ret = True
                break

        return ret

    def sum_unmarked(self):
        sum = 0
        for i in range(5):
            for j in range(5):
                if self.marked[i][j] == 0:
                    sum += self.numbers[i][j]

        return sum




def task1(boards, numbers):
    for num in numbers:
        for board in boards:
            board.mark(num)
            if board.check_complete():
                return board.sum_unmarked() * num


def task2(boards, numbers):
    complete_count = 0
    total = len(boards)
    complete_idx = []
    for num in numbers:
        for i in range(len(boards)):
            if i in complete_idx:
                continue
            boards[i].mark(num)
            if boards[i].check_complete():
                complete_count += 1
                complete_idx.append(i)
                if complete_count == total:
                    return boards[i].sum_unmarked() * num


with open("Input") as f:
    content = [line.strip() for line in f.readlines()]

numbers = [int(num) for num in content[0].split(",")]
boards = []
cur_board = []
for i in range(1, len(content)):
    nums = [int(num) for num in content[i].split()]
    if len(nums) > 1:
            cur_board.append(nums)
    else: 
        if len(cur_board) > 1:
            boards.append(Board(cur_board))
            cur_board = []


print("Task 1:", task1(boards[:], numbers))
print("Task 2:", task2(boards[:], numbers))