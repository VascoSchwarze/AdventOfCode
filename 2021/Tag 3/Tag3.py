def task1(nums):
    digit_count = len(nums[0])
    one_count = [0 for i in range(digit_count)]
    zero_count = [0 for i in range(digit_count)]
    for bits in nums:
        for i in range(len(bits)):
            if bits[i] == "0":
                zero_count[i] += 1
                continue
            if bits[i] == "1":
                one_count[i] += 1

    gamma = ""
    epsilon = ""
    for i in range(digit_count):
        if one_count[i] > zero_count[i]:
            gamma += "1"
            epsilon += "0"
        else:
            gamma += "0"
            epsilon += "1"
    
    gamma_dec = int(gamma, 2)
    epsilon_dec = int(epsilon, 2)
    return gamma_dec * epsilon_dec


def task2(nums):
    return oxygen(nums) * co2(nums)

def oxygen(nums):
    digit_count = len(nums[0])
    for i in range(digit_count):
        cur_bit = str(get_most_common_bit_at(nums, i, 1))
        nums = [num for num in nums if num[i] == cur_bit]
        if len(nums) == 1:
            break

    return int(nums[0], 2)

def co2(nums):
    digit_count = len(nums[0])
    for i in range(digit_count):
        cur_bit = str(0 if get_most_common_bit_at(nums, i, 1) == 1 else 1)
        nums = [num for num in nums if num[i] == cur_bit]
        if len(nums) == 1:
            break

    return int(nums[0], 2)

def get_most_common_bit_at(nums, pos, ifEqual):
    ones = 0
    zeroes = 0
    for bits in nums:
        if bits[pos] == "0":
            zeroes += 1
        if bits[pos] == "1":
            ones += 1
    if ones == zeroes:
        return ifEqual
    elif ones > zeroes:
        return 1
    else: return 0


with open("Input") as f:
    content = [line.strip() for line in f.readlines()]

print("Task 1:", task1(content))
print("Task 2:", task2(content))