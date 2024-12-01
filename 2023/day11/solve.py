from typing import List

def expand_universe(universe: List[List[str]]):
    i = 0
    # rows
    while i < len(universe):
        if "#" not in universe[i]:
            new_line = ["."] * len(universe[0])

            universe.insert(i, new_line)
            i += 1
        i += 1
    
    def is_column_empty(index: int) -> bool:
        for line in universe:
            if line[index] == "#":
                return False
        return True

    def add_column(index: int):
        for i in range(len(universe)):
            universe[i].insert(index, ".")

    # columns
    i = 0
    while i < len(universe[0]):
        if is_column_empty(i):
            add_column(i)
            i += 1
        i += 1

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
expand_universe(universe)

galaxies = get_galaxies(universe)
pairs = get_galaxie_pairs(galaxies)

sum_shortest_path = 0
for pair in pairs:
    galaxie1, galaxie2 = pair

    sum_shortest_path += abs(galaxie1[0] - galaxie2[0]) + abs(galaxie1[1] - galaxie2[1])
print(sum_shortest_path)