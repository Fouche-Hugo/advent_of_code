file = open("day2/input.txt", "r")

lines = file.readlines()

sum_set_powers = 0
for line in lines:
    id_end_index = line.find(":")

    # get the max amount of cubes for each color
    max_cubes_amount = {
        "red": 0,
        "green": 0,
        "blue": 0
    }

    records = [record.split(",") for record in line[id_end_index+1:].strip().split(";")]
    
    for record in records:
        for color_amount in record:
            (number, color) = color_amount.strip().split(" ")
            
            if max_cubes_amount[color] < int(number):
                max_cubes_amount[color] = int(number)
    
    sum_set_powers += max_cubes_amount["blue"] * max_cubes_amount["red"] * max_cubes_amount["green"]

print(sum_set_powers)