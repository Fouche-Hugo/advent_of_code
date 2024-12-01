def all_zeros(sequence):
    for number in sequence:
        if number != 0:
            return False
    return True

file = open("day9/input.txt")

lines = file.readlines()

sum_extrapolated_values = 0
for line in lines:
    line = line.strip()

    values = [int(value) for value in line.split(" ")]
    sequences = [values]

    while not all_zeros(sequences[len(sequences) - 1]):
        new_sequence = []

        for i in range(1, len(sequences[len(sequences) - 1])):
            new_sequence.append(sequences[len(sequences) - 1][i] - sequences[len(sequences) - 1][i - 1])
        sequences.append(new_sequence)
    # find the new number
    sequences[len(sequences) - 1].append(0)
    for i in range(1, len(sequences)):
        last_sequence = sequences[len(sequences) - i]
        first_number_from_last_sequence = last_sequence[0]

        index_current_sequence = len(sequences) - 1 - i
        len_current_sequence = len(sequences[index_current_sequence])
        first_number_from_current_sequence = sequences[index_current_sequence][0]
        
        sequences[index_current_sequence].insert(0, first_number_from_current_sequence - first_number_from_last_sequence)
    
    extrapolated_value = sequences[0][0]
    sum_extrapolated_values += extrapolated_value

print(sum_extrapolated_values)