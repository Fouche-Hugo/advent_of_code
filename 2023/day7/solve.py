from collections import Counter
from functools import cmp_to_key

hand_types = {
    "High card": 0,
    "One pair": 1,
    "Two pair": 2,
    "Three of a kind": 3,
    "Full house": 4,
    "Four of a kind": 5,
    "Five of a kind": 6
}

card_powers = {
    "2": 0,
    "3": 1,
    "4": 2,
    "5": 3,
    "6": 4,
    "7": 5,
    "8": 6,
    "9": 7,
    "T": 8,
    "J": 9,
    "Q": 10,
    "K": 11,
    "A": 12,
}

def assign_type(hand):
    letters = Counter(hand)
    
    max_of_a_kind = max(letters.values())
    
    if max_of_a_kind == 5:
        return hand_types["Five of a kind"]
    if max_of_a_kind == 4:
        return hand_types["Four of a kind"]
    if max_of_a_kind == 3:
        # Full house of Three of a kind
        if len(letters) == 2:
            return hand_types["Full house"]
        return hand_types["Three of a kind"]
    if max_of_a_kind == 2:
        # Two pair or one pair
        if len(letters) == 3:
            return hand_types["Two pair"]
        return hand_types["One pair"]
    return hand_types["High card"]

def hand_comparison(hand_left, hand_right):
    letters_left = hand_left[0]
    letters_right = hand_right[0]
    
    for i in range(len(letters_left)):
        if letters_left[i] == letters_right[i]:
            continue
        
        if card_powers[letters_left[i]] < card_powers[letters_right[i]]:
            return -1
        return 1
        
    return -1

file = open("day7/input.txt")

lines = file.readlines()

# list of list of hands, where each list correpond to a hand type
hands = [[] for _ in range(len(hand_types))]

for line in lines:
    line = line.strip()
    
    cards, bid = line.split(" ")
    
    type_index = assign_type(cards)
    
    hands[type_index].append((cards, int(bid)))

# order each list
for i in range(len(hands)):
    hands[i].sort(key=cmp_to_key(hand_comparison))

total_winnings = 0
current_rank = 0
for i in range(len(hands)):
    for j in range(len(hands[i])):
        current_rank += 1
        total_winnings += current_rank * hands[i][j][1]

print(total_winnings)