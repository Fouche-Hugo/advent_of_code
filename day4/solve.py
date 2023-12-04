file = open("day4/input.txt")

lines = file.readlines()

total_points = 0
for line in lines:
    start_index = line.find(":")
    
    line = line[start_index+1:]
    
    winning_numbers = line[:line.find("|")].strip().split(" ")
    numbers = line[line.find("|")+1:].strip().split(" ")
    
    points = 0
    
    for number in numbers:
        if number in winning_numbers and number != "":
            points = 1 if points == 0 else 2 * points
    
    total_points += points
    
print(total_points)