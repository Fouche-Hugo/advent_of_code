file = open("day2/input.txt", "r")

lines = file.readlines()

objective = {
    "red": 12,
    "green": 13,
    "blue": 14
}

sum_valid_ids = 0
for line in lines:
    # start of the game id at index 5 (Game 1) -> 1 is at index 5
    id_start_index = 5
    id_end_index = line.find(":")

    id = line[id_start_index:id_end_index]

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
    
    valid_id = True
    for color in max_cubes_amount:
        if max_cubes_amount[color] > objective[color]:
            valid_id = False
            break
    
    if valid_id:
        sum_valid_ids += int(id)

print(sum_valid_ids)