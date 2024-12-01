from typing import List

def is_column_empty(universe: List[List[str]], index: int) -> bool:
    for line in universe:
        if line[index] == "#":
            return False
    return True

def is_row_empty(universe: List[List[str]], index: int) -> bool:
    if "#" in universe[index]:
        return False
    return True

def calculate_shortest_path(universe: List[List[str]], galaxie1: tuple[int], galaxie2: tuple[int]) -> int:
    # rows difference
    x_left, x_right = galaxie1[0], galaxie2[0]

    if x_left > x_right:
        x_left, x_right = x_right, x_left
    
    row_difference = 0
    for i in range(x_left + 1, x_right + 1):
        if is_row_empty(universe, i):
            row_difference += 1000000
        else:
            row_difference += 1
    
    # column difference
    y_up, y_down = galaxie1[1], galaxie2[1]

    if y_up > y_down:
        y_up, y_down = y_down, y_up
    
    column_difference = 0
    for i in range(y_up + 1, y_down + 1):
        if is_column_empty(universe, i):
            column_difference += 1000000
        else:
            column_difference += 1
    
    return row_difference + column_difference

def get_galaxies(universe: List[List[str]]) -> List[tuple[int]]:
    galaxies = []
    for i, line in enumerate(universe):
        for j in range(len(line)):
            if line[j] == "#":
                galaxies.append((i, j))
    return galaxies

def get_galaxie_pairs(galaxies: List[tuple[int]]) -> List[tuple[tuple[int]]]:
    pairs = []
    for i in range(len(galaxies)):
        for j in range(i + 1, len(galaxies)):
            pairs.append((galaxies[i], galaxies[j]))
    return pairs

file = open("day11/input.txt")
universe = [[step for step in line.strip()] for line in file.readlines()]

galaxies = get_galaxies(universe)
pairs = get_galaxie_pairs(galaxies)

sum_shortest_path = 0
for pair in pairs:
    galaxie1, galaxie2 = pair

    sum_shortest_path += calculate_shortest_path(universe, galaxie1, galaxie2)
print(sum_shortest_path)