

with open('input.txt') as f:
    data = f.readlines()

cols = len(data[0])-1

gamma = ""
epsilon = ""
for i in range(0,cols):
    ones = len([l[i] for l in data if l[i] == '1'])
    zeros = len([l[i] for l in data if l[i] == '0'])

    gamma += "1" if ones > zeros else "0"
    epsilon += "1" if ones < zeros else "0"

print(int(gamma, 2) * int(epsilon,2))