file = open("day5/input.txt")

lines = file.readlines()

seeds = [int(seed) for seed in lines[0][6:].strip().split(" ")]
convertors = []
current_convertor = []

for line in lines[3:]:
    # ignore empty line
    if not line[0].isalnum():
        continue

    # if the first character is a digit => read the line
    if line[0].isdigit():
        current_convertor.append([int(value) for value in line.strip().split(" ")])
    else:
        convertors.append(current_convertor)
        current_convertor = []
convertors.append(current_convertor)

lowest_location = -1
for seed in seeds:
    current_number = seed

    for convertor in convertors:
        for values in convertor:
            if current_number >= values[1] and current_number <= values[1] + values[2]:
                current_number = values[0] + current_number - values[1]
                break
    
    # final test
    if lowest_location == -1 or lowest_location > current_number:
        lowest_location = current_number

print(lowest_location)
