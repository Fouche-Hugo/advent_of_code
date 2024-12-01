def calculate_distance(total_time, time_pressed):
    speed = time_pressed
    distance = speed * (total_time - time_pressed)
    
    return distance

file = open("day6/input.txt")

lines = file.readlines()

times = [int(info.strip()) for info in lines[0][5:].strip().split(" ") if info != ""]
distances = [int(info.strip()) for info in lines[1][10:].strip().split(" ") if info != ""]

total_number = 1
for i in range(len(times)):
    total_time = times[i]
    objective = distances[i]
    numbers = 0
    
    for j in range(total_time):
        distance = calculate_distance(total_time, j)
        
        if distance > objective:
            numbers += 1
    
    total_number *= numbers

print(total_number)