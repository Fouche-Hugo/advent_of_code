file = open("day1/input.txt", "r")

lines = file.readlines()

calibrations_sum = 0
for line in lines:
    start, end = 0, len(line) - 1

    # find first digit
    while not line[start].isdigit():
        start += 1
    
    # find last digit
    while not line[end].isdigit():
        end -= 1
    
    calibrations_sum += int(line[start] + line[end])

print(calibrations_sum)