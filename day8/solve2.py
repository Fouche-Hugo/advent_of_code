def pgcd(a, b):  
    if a < b:
        a, b = b, a
    
    if b == 0:
        return b

    r = a % b
    while r != 0:
        a, b = b, r
        r = a % b
    
    return b

def ppcm(a, b):
    return a * b // pgcd(a, b)

file = open("day8/input.txt")

lines = file.readlines()

directions_indexes = {
    "L": 0,
    "R": 1
}

directions = lines[0].strip()

nodes = {}
for line in lines[2:]:
    line = line.strip()

    node_name, nodes_derived = [value.strip() for value in line.split("=")]
    nodes_derived = [value.strip().replace("(", "").replace(")", "") for value in nodes_derived.split(",")]

    nodes[node_name] = nodes_derived

current_node_names = [key for key in nodes.keys() if key[2] == 'A']
objective_nodes = [key for key in nodes.keys() if key[2] == 'Z']

nb_of_steps = [0 for i in range(len(current_node_names))]

for i, name in enumerate(current_node_names):
    current_node_name = name
    while current_node_name[2] != "Z":
        nb_of_steps[i] += 1

        next_direction = directions_indexes[directions[(nb_of_steps[i] - 1) % len(directions)]]
        current_node_name = nodes[current_node_name][next_direction]

current_ppcm = ppcm(nb_of_steps[0], nb_of_steps[1])

for i in range(2, len(nb_of_steps)):
    current_ppcm = ppcm(current_ppcm, nb_of_steps[i])

print(current_ppcm)