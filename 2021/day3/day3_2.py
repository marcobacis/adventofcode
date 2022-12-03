

with open('input.txt') as f:
    data = f.readlines()

cols = len(data[0])-1
# Find oxygen generator and CO2 scrubber ratings

def find_rating(data, keep, not_keep):
    numbers = data
    rating = 0
    for i in range(0, cols):
        ones = len([l[i] for l in numbers if l[i] == '1'])
        zeros = len([l[i] for l in numbers if l[i] == '0'])

        most_common = keep if ones >= zeros else not_keep
        numbers = [n for n in numbers if n[i] == most_common]
        if len(numbers) == 1:
            rating = int(numbers[0], 2)
            break
    return rating

def oxygen_rating(data):
    return find_rating(data, "1", "0")

def co2_rating(data):
    return find_rating(data, "0","1")

print(oxygen_rating(data) * co2_rating(data))