file = open("day3/input.txt", "r")

lines = file.readlines()

total_sum = 0
for i, line in enumerate(lines):
    # find the numbers
    line = line.strip()
    j = 0

    while j < len(line):
        if line[j].isdigit():
            start_index = j
            end_index = start_index
            # find the end of the number
            while end_index < len(line) and line[end_index].isdigit():
                end_index += 1
            
            number = int(line[start_index:end_index])

            # check if the number is valid
            if start_index > 0 and line[start_index-1] != ".":
                # symbol of the left
                total_sum += number
            elif end_index < len(line) - 1 and line[end_index] != ".":
                # symbol on the right
                total_sum += number
            else:
                valid = False
                # check upper line
                if i > 0:
                    for k in range(max(0, start_index - 1), min(len(line), end_index + 1)):
                        if lines[i - 1].strip()[k] != "." and not lines[i - 1].strip()[k].isdigit():
                            total_sum += number
                            valid = True
                            break
                
                # check below line
                if i < len(lines) - 1 and not valid:
                    for k in range(max(0, start_index - 1), min(len(line), end_index + 1)):
                        if lines[i + 1].strip()[k] != "." and not lines[i + 1].strip()[k].isdigit():
                            total_sum += number
                            break

            j = end_index

        j += 1

print(total_sum)