from collections import Counter

def read_input(filename):
    with open(filename,'r') as f:
        lines = f.read().splitlines()
        template = lines[0]
        rules = {l[:2]: l[-1] for l in lines[2:]} #Parse each line "XY -> Z"        
        polymer = {k: template.count(k) for k in rules.keys()}

        return polymer, rules

# Old version -> create string
#def polymer_pass(template, rules):
#    new_polymer = ""
#    for i in range(len(template)-1):
#        new_polymer += template[i]
#
#        pair = str(template[i]) + str(template[i+1])
#
#        if pair in rules:
#            new_polymer += rules[pair]
#
#    new_polymer += template[-1] 
#
#    return new_polymer

# New version, use counter map and transform counts based on rules
def polymer_pass(polymer, rules):
    new_polymer = polymer.copy()
    for (rule, c) in polymer.items():
        if c > 0:
            new_polymer[rule] -= c
            new_polymer[rule[0] + rules[rule]] += c
            new_polymer[rules[rule] + rule[1]] += c

    return new_polymer

def max_min_difference(polymer):
    counts = {k: 0 for k in set(''.join(polymer.keys()))}

    for rule, count in polymer.items():
        counts[rule[0]] += count / 2
        counts[rule[1]] += count / 2
    
    return int(max(counts.values()) - min(counts.values()))

polymer, rules = read_input("input.txt")

for i in range(40):
    polymer = polymer_pass(polymer, rules)
    if i == 9:
        counter = Counter(polymer)
        print("Part 1: {}".format(max_min_difference(polymer)))

print("Part 2: {}".format(max_min_difference(polymer)))