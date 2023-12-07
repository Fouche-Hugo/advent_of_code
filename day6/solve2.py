def calculate_distance(total_time, time_pressed):
    speed = time_pressed
    distance = speed * (total_time - time_pressed)
    
    return distance

file = open("day6/input.txt")

lines = file.readlines()

times = [info.strip() for info in lines[0][5:].strip().split(" ") if info != ""]
distances = [info.strip() for info in lines[1][10:].strip().split(" ") if info != ""]

total_time = int("".join(times))
objective = int("".join(distances))

total_number = 0

for j in range(total_time):
    distance = calculate_distance(total_time, j)
    
    if distance > objective:
        total_number += 1

print(total_number)