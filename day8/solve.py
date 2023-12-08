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

current_node_name = "AAA"
nb_of_steps = 0
while current_node_name != "ZZZ":
    nb_of_steps += 1

    next_direction = directions_indexes[directions[(nb_of_steps - 1) % len(directions)]]
    current_node_name = nodes[current_node_name][next_direction]
print(nb_of_steps)