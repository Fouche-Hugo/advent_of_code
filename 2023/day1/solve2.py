file = open("day1/input.txt", "r")

lines = file.readlines()

letter_digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]

calibrations_sum = 0
for line in lines:
    start_raw_digit, end_raw_digit = 0, len(line) - 1
    start_letter_digit, end_letter_digit = 0, len(line) - 1
    final_digits = ""
    
    # find first digit
    while not line[start_raw_digit].isdigit():
        start_raw_digit += 1
    
    # check if it's the real first digit
    start_letter_digit, letter_digit = -1, -1
    for i, word in enumerate(letter_digits):
        starting_index = line.find(word)
        
        if start_letter_digit == -1 or (starting_index >= 0 and starting_index < start_letter_digit):
            start_letter_digit = starting_index
            letter_digit = i + 1
    
    if start_letter_digit != -1 and start_letter_digit < start_raw_digit:
        final_digits += str(letter_digit)
    else:
        final_digits += line[start_raw_digit]
    
    # find last digit
    while not line[end_raw_digit].isdigit():
        end_raw_digit -= 1
    
    # check if it's the real last digit
    end_letter_digit, letter_digit = -1, -1
    for i, word in enumerate(letter_digits):
        ending_index = line.rfind(word)
        
        if end_letter_digit == -1 or (ending_index >= 0 and ending_index > end_letter_digit):
            end_letter_digit = ending_index
            letter_digit = i + 1
    
    if end_letter_digit != -1 and end_letter_digit > end_raw_digit:
        final_digits += str(letter_digit)
    else:
        final_digits += line[end_raw_digit]
    
    calibrations_sum += int(final_digits)

print(calibrations_sum)