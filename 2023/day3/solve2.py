file = open("day3/input.txt", "r")

lines = file.readlines()

total_sum = 0
for i, line in enumerate(lines):
    # find the stars
    line = line.strip()

    for j, letter in enumerate(line):
        if letter == "*":
            gears = []

            # check left and right if there are digits
            if j > 0 and line[j - 1].isdigit():
                # left
                start_digit = j

                while start_digit > 0 and line[start_digit - 1].isdigit():
                    start_digit -= 1
                
                gears.append(line[start_digit:j])

            if j + 1 < len(line) and line[j + 1].isdigit():
                # right
                end_digit = j + 1

                while end_digit + 1 < len(line) and line[end_digit].isdigit():
                    end_digit += 1
                
                gears.append(line[j+1:end_digit])

            # check the upper row
            if i > 0:
                for k in range(max(0, j - 1), min(len(line), j + 2)):
                    if lines[i - 1].strip()[k].isdigit():
                        # find the number
                        start_index = end_index = k

                        while start_index > 0 and lines[i - 1].strip()[start_index - 1].isdigit():
                            start_index -= 1
                        
                        while end_index < len(line) and lines[i - 1].strip()[end_index].isdigit():
                            end_index += 1
                        
                        gears.append(lines[i - 1].strip()[start_index:end_index])
                        
                        if start_index <= j < end_index:
                            break
            
            # check the below row
            if i < len(lines) - 1:
                for k in range(max(0, j - 1), min(len(line), j + 2)):
                    if lines[i + 1].strip()[k].isdigit():
                        # find the number
                        start_index = end_index = k

                        while start_index > 0 and lines[i + 1].strip()[start_index - 1].isdigit():
                            start_index -= 1
                        
                        while end_index < len(line) and lines[i + 1].strip()[end_index].isdigit():
                            end_index += 1
                        
                        gears.append(lines[i + 1].strip()[start_index:end_index])
                        
                        if start_index <= j < end_index:
                            break
            
            # check if there is exactly two numbers
            if len(gears) == 2:
                gear_ratio = int(gears[0]) * int(gears[1])
                total_sum += gear_ratio

print(total_sum)