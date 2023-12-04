def task1(entries):
    count = 0
    for entry in entries:
        for num in entry['output']:
            if len(num) == 2 or len(num) == 4 or len(num) == 3 or len(num) == 7:
                count += 1

    return count


def task2(entries):
    sum = 0
    for entry in entries:
        input = entry['input'].copy()
        segments = {}
        for i in range(7):
            segments[i] = set('abcdefg')
        
        numbers = {}
        for num in entry['input']:
            if len(num) == 2:
                for seg in num_to_segments(1):
                    segments[seg] = segments[seg].intersection(set(num))
                numbers[1] = set(num)
                input.remove(num)
            if len(num) == 3:
                for seg in num_to_segments(7):
                    segments[seg] = segments[seg].intersection(set(num))
                numbers[7] = set(num)
                input.remove(num)
            if len(num) == 4:
                for seg in num_to_segments(4):
                    segments[seg] = segments[seg].intersection(set(num))
                numbers[4] = set(num)
                input.remove(num)
            if len(num) == 7:
                for seg in num_to_segments(8):
                    segments[seg] = segments[seg].intersection(set(num))
                numbers[8] = set(num)
                input.remove(num)


        for l in combinations([2, 3, 5], [i for i in input if len(i) == 5]):
            for m in combinations([0, 6, 9], [i for i in input if len(i) == 6]):
                # debug = False
                # if ('gcdfa', 2) in (l+m) and ('fbcad', 3) in (l+m) and ('cagedb', 0) in (l+m) and ('cdfgeb', 6) in (l+m):
                #     debug = True
                segments_copy = segments.copy()
                # if debug:
                #     print(segments_copy)
                for segs, num in (l + m):
                    for seg in num_to_segments(num):
                        segments_copy[seg] = segments_copy[seg].intersection(set(segs))

                # if debug:
                #     print(segments_copy)

                for i in range(7):
                    for key in segments_copy.keys():
                        if len(segments_copy[key]) == 1:
                            for ikey in segments_copy.keys():
                                if ikey == key:
                                    continue
                                segments_copy[ikey].difference_update(set(segments_copy[key]))

                # if debug:       
                #     print(segments_copy)
                valid = True
                for key in segments_copy.keys():
                    if len(segments_copy[key]) != 1:
                        valid = False
                        break
                

                if valid:
                    for segs, num in (l + m):
                        numbers[num] = set(segs)

                    break 
                
            if valid:
                break
        
        # print(segments_copy)
        # print(numbers)

        output = entry['output']
        for i in range(4):
            for num, segs in numbers.items():
                if set(output[i]) == set(segs):
                    sum += pow(10, 3 - i) * num

    return sum

def num_to_segments(number):
    segs = {'0': [0, 1, 2, 4, 5, 6],
            '1': [2, 5],
            '2': [0, 2, 3, 4, 6],
            '3': [0, 2, 3, 5, 6],
            '4': [1, 2, 3, 5],
            '5': [0, 1, 3, 5, 6],
            '6': [0, 1, 3, 4, 5, 6],
            '7': [0, 2, 5],
            '8': [0, 1, 2, 3, 4, 5, 6],
            '9': [0, 1, 2, 3, 5, 6]}

    return segs[str(number)]


def combinations(nums, strs):
    c = []

    str_perms = itertools.permutations(strs, len(strs))
    for perm in str_perms:
        zipped = zip(perm, nums)
        c.append(list(zipped))

    return c

import itertools

with open("Input") as f:
    content = [line.strip() for line in f.readlines()]

entries = []
for line in content:
    entry = {}
    input, output = line.split("|")
    input = [num.strip() for num in input.split()]
    output = [num.strip() for num in output.split()]
    entry['input'] = input
    entry['output'] = output
    entries.append(entry)


print("Task 1:", task1(entries))
print("Task 2:", task2(entries))