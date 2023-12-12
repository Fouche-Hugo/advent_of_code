def is_arrangement_valid(format1_replaced: str, format2: str):
    i = 0
    current_group = 0
    while i < len(format1_replaced):
        if format1_replaced[i] == "#":
            if current_group == len(format2):
                return False

            # find the end of the group
            j = i + 1
            while j < len(format1_replaced) and format1_replaced[j] == "#":
                j += 1
            
            if j - i != int(format2[current_group]):
                return False
            
            current_group += 1
            i = j - 1
        
        i += 1
    
    if current_group == len(format2):
        return True
    return False

def generate_arrangements(format1: str):
    if len(format1) == 0:
        return []
    
    index = format1.find("?")

    if index == -1:
        return [format1]
    
    arrangements = []
    
    list_format1 = list(format1)
    list_format1[index] = "."
    arrangements.extend(generate_arrangements("".join(list_format1)))
    
    list_format1[index] = "#"
    arrangements.extend(generate_arrangements("".join(list_format1)))

    return arrangements

file = open("day12/input.txt")

lines = [line.strip() for line in file.readlines()]

sum_valid_arrangements = 0
for line in lines:
    format1, format2 = line.split()
    format2 = format2.split(",")

    arrangements = generate_arrangements(format1)

    for arrangement in arrangements:
        if is_arrangement_valid(arrangement, format2):
            sum_valid_arrangements += 1

print(sum_valid_arrangements)