

def line_score(line):
    stack = []
    brackets = {'(': ')', '[': ']', '{': '}', '<': '>' }
    scores = {')': 3, ']': 57, '}': 1197, '>': 25137 }

    opening = list(brackets.keys())
    closing = list(brackets.values())

    for c in line:
        if c in opening:
            stack.append(c)
        elif c in closing:
            opened = stack.pop()
            if c != brackets[opened]:
                return scores[c]
    
    return 0


with open('inputs.txt') as f:
    lines = f.read().splitlines()

print(sum([line_score(l) for l in lines]))