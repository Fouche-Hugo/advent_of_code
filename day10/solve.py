def find_starting_position(terrain):
    for i in range(len(terrain)):
        for j in range(len(terrain[0])):
            if terrain[i][j] == "S":
                return (i, j)
    return None

def find_pipe_entries_from_starting_point(terrain, starting_point):
    y, x = starting_point
    entries = []

    if x > 0:
        # left
        from_direction = to_direction_to_from_direction[WEST]
        new_direction = from_pipes[from_direction].get(terrain[y][x - 1])
        if new_direction:
            entries.append(((y, x - 1), from_direction))
    
    if y > 0:
        # up
        from_direction = to_direction_to_from_direction[NORTH]
        new_direction = from_pipes[from_direction].get(terrain[y - 1][x])
        if new_direction:
            entries.append(((y - 1, x), from_direction))
    
    if y + 1 < len(terrain):
        # down
        from_direction = to_direction_to_from_direction[SOUTH]
        new_direction = from_pipes[from_direction].get(terrain[y + 1][x])
        if new_direction:
            entries.append(((y + 1, x), from_direction))
    
    if x + 1 < len(terrain[0]):
        # right
        from_direction = to_direction_to_from_direction[EAST]
        new_direction = from_pipes[from_direction].get(terrain[y][x + 1])
        if new_direction:
            entries.append(((y, x + 1), from_direction))

    return entries

def follow_pipe(terrain, position, from_direction):
    current_pipe_type = terrain[position[0]][position[1]]

    new_direction = from_pipes[from_direction][current_pipe_type]

    position = (position[0] + directions[new_direction][0], position[1] + directions[new_direction][1])

    return (position, to_direction_to_from_direction[new_direction])

WEST = "WEST"
EAST = "EAST"
NORTH = "NORTH"
SOUTH = "SOUTH"

directions = {
    # (y, x)
    WEST: (0, -1),
    EAST: (0, 1),
    NORTH: (-1, 0),
    SOUTH: (1, 0)
}

# convert the current direction to the direction the pipe came from
to_direction_to_from_direction = {
    WEST: EAST,
    EAST: WEST,
    NORTH: SOUTH,
    SOUTH: NORTH
}

from_pipes = {
    WEST: {
        "-": EAST,
        "J": NORTH,
        "7": SOUTH
    },
    EAST: {
        "-": WEST,
        "L": NORTH,
        "F": SOUTH
    },
    NORTH: {
        "|": SOUTH,
        "L": EAST,
        "J": WEST
    },
    SOUTH: {
        "|": NORTH,
        "7": WEST,
        "F": EAST
    }
}

file = open("day10/input.txt")

terrain = [line.strip() for line in file.readlines()]
start = find_starting_position(terrain)
entries = find_pipe_entries_from_starting_point(terrain, start)

entrie1, entrie2 = entries

steps = 1
while entrie1[0] != entrie2[0]:
    entrie1 = follow_pipe(terrain, entrie1[0], entrie1[1])
    entrie2 = follow_pipe(terrain, entrie2[0], entrie2[1])

    steps += 1
print(steps)