file = open("day5/input.txt")

lines = file.readlines()

seeds = [int(seed) for seed in lines[0][6:].strip().split(" ")]
seeds_to_soil = []

line_index = 3

while lines[line_index][0].isdigit():
    seeds_to_soil.append([int(value) for value in lines[line_index].strip().split(" ")])

    line_index += 1

soil_to_fertilizer = []

line_index += 2

while lines[line_index][0].isdigit():
    soil_to_fertilizer.append([int(value) for value in lines[line_index].strip().split(" ")])

    line_index += 1

fertilizer_to_water = []

line_index += 2

while lines[line_index][0].isdigit():
    fertilizer_to_water.append([int(value) for value in lines[line_index].strip().split(" ")])

    line_index += 1

water_to_light = []

line_index += 2

while lines[line_index][0].isdigit():
    water_to_light.append([int(value) for value in lines[line_index].strip().split(" ")])

    line_index += 1

light_to_temperature = []

line_index += 2

while lines[line_index][0].isdigit():
    light_to_temperature.append([int(value) for value in lines[line_index].strip().split(" ")])

    line_index += 1

temperature_to_humidity = []

line_index += 2

while lines[line_index][0].isdigit():
    temperature_to_humidity.append([int(value) for value in lines[line_index].strip().split(" ")])

    line_index += 1

humidity_to_location = []

line_index += 2

while line_index < len(lines) and lines[line_index][0].isdigit():
    humidity_to_location.append([int(value) for value in lines[line_index].strip().split(" ")])

    line_index += 1

lowest_location = -1

for seed in seeds:
    # find the soil
    soil_number = -1
    for soil in seeds_to_soil:
        if seed >= soil[1] and seed <= soil[1] + soil[2]:
            soil_number = soil[0] + seed - soil[1]
            break
    
    if soil_number == -1:
        soil_number = seed
    
    # find the fertilizer
    fertilizer_number = -1
    for fertilizer in soil_to_fertilizer:
        if soil_number >= fertilizer[1] and soil_number <= fertilizer[1] + fertilizer[2]:
            fertilizer_number = fertilizer[0] + soil_number - fertilizer[1]
            break
    
    if fertilizer_number == -1:
        fertilizer_number = soil_number
    
    # find the water
    water_number = -1
    for water in fertilizer_to_water:
        if fertilizer_number >= water[1] and fertilizer_number <= water[1] + water[2]:
            water_number = water[0] + fertilizer_number - water[1]
            break
    
    if water_number == -1:
        water_number = fertilizer_number
    
    # find the light
    light_number = -1
    for light in water_to_light:
        if water_number >= light[1] and water_number <= light[1] + light[2]:
            light_number = light[0] + water_number - light[1]
            break
    
    if light_number == -1:
        light_number = water_number
    
    # find the temperature
    temperature_number = -1
    for temperature in light_to_temperature:
        if light_number >= temperature[1] and light_number <= temperature[1] + temperature[2]:
            temperature_number = temperature[0] + light_number - temperature[1]
            break
    
    if temperature_number == -1:
        temperature_number = light_number
    
    # find the humidity
    humidity_number = -1
    for humidity in temperature_to_humidity:
        if temperature_number >= humidity[1] and temperature_number <= humidity[1] + humidity[2]:
            humidity_number = humidity[0] + temperature_number - humidity[1]
            break
    
    if humidity_number == -1:
        humidity_number = temperature_number
    
    # find the location
    location_number = -1
    for location in humidity_to_location:
        if humidity_number >= location[1] and humidity_number <= location[1] + location[2]:
            location_number = location[0] + humidity_number - location[1]
            break
    
    if location_number == -1:
        location_number = humidity_number
    
    # final test
    if lowest_location == -1 or lowest_location > location_number:
        lowest_location = location_number

print(lowest_location)