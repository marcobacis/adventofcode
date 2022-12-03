
# 1 segment : none
# 2 segments: 1
# 3 segments: 7
# 4 segments: 4
# 5 segments: 2,3,5,9
# 6 segments: 0,6
# 7 segments: 8

# First puzzle: count the "digits" with a unique number of segments

# Read inputs
with open('inputs.txt','r') as f:
    inputs = f.readlines()

outputs = [l.split(" | ")[1].split() for l in inputs]

unique_digits = [len([word for word in words if len(word) in [2,3,4,7]]) for words in outputs]

print(sum(unique_digits))