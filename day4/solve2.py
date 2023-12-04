file = open("day4/input.txt")

lines = file.readlines()

cards = {}

for line in lines:
    start_index = line.find(":")
    
    card_id = int(line[5:start_index])
    
    if cards.get(card_id):
        cards[card_id] += 1
    else:
        cards[card_id] = 1
    
    winning_numbers = line[:line.find("|")].strip().split(" ")
    numbers = line[line.find("|")+1:].strip().split(" ")
    
    matching_numbers = 0
    
    for number in numbers:
        if number in winning_numbers and number != "":
            matching_numbers += 1
    
    for i in range(1, matching_numbers + 1):
        if cards.get(card_id + i):
            cards[card_id + i] += cards[card_id]
        else:
            cards[card_id + i] = cards[card_id]

total_number_cards = 0
for card in cards:
    total_number_cards += cards[card]
print(total_number_cards)