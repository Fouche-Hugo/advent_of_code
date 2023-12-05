def build_convertors(lines):
    convertors = []
    current_convertor = []

    for line in lines:
        # ignore empty line
        if not line[0].isalnum():
            continue

        # if the first character is a digit => read the line
        if line[0].isdigit():
            current_convertor.append([int(value) for value in line.strip().split(" ")])
        else:
            convertors.append(current_convertor)
            current_convertor = []
    convertors.append(current_convertor)

    return convertors

def find_seed_location(seed, convertors):
    current_number = seed

    for convertor in convertors:
        for values in convertor:
            if current_number >= values[1] and current_number <= values[1] + values[2]:
                current_number = values[0] + current_number - values[1]
                break
    
    return current_number

def convert_range(source, convertor):
    # source: [start, end]
    destinations = []
    sources = [source]

    i = 0

    while i < len(sources):
        for convertor_range in convertor:
            # test if there is numbers in common between source and convertor
            if not (sources[i][0] >= convertor_range[1] + convertor_range[2] or sources[i][1] < convertor_range[1]):
                start_source = max(sources[i][0], convertor_range[1])
                end_source = min(sources[i][1], convertor_range[1] + convertor_range[2] - 1)

                start_destination = convertor_range[0] + start_source - convertor_range[1]
                end_destination = convertor_range[0] - (convertor_range[1] - end_source)

                destinations.append([start_destination, end_destination])

                if start_source > sources[i][0]:
                    sources.append([sources[i][0], start_source - 1])
                if end_source < sources[i][1]:
                    sources.append([end_source + 1, sources[i][1]])
                
                sources.pop(i)
                i -= 1
                break

        i += 1
    destinations.extend(sources)
    return destinations

def convert_ranges(sources, convertor):
    destinations = []

    for source in sources:
        destinations.extend(convert_range(source, convertor))
    
    return destinations

file = open("day5/input.txt")

lines = file.readlines()

seeds_infos = [int(seed) for seed in lines[0][6:].strip().split(" ")]
seeds = []
for i in range(0, len(seeds_infos), 2):
    # seeds contains a range with minimum and maximum
    seeds.append([seeds_infos[i], seeds_infos[i] + seeds_infos[i + 1] - 1])

convertors = build_convertors(lines[3:])

lowest_location = -1

for seed_range in seeds:
    destinations = [seed_range]

    for convertor in convertors:
        destinations = convert_ranges(destinations, convertor)

    lowest_starter = min(destinations, key=lambda x: x[0])

    if lowest_location == -1 or lowest_location > lowest_starter[0]:
        lowest_location = lowest_starter[0]

print(lowest_location)