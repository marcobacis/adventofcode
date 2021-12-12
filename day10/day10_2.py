

def line_score(line):
    stack = []
    brackets = {'(': ')', '[': ']', '{': '}', '<': '>' }
    scores = {'(': 1, '[': 2, '{': 3, '<': 4 }

    opening = list(brackets.keys())
    closing = list(brackets.values())

    # Parse
    for c in line:
        if c in opening:
            stack.append(c)
        elif c in closing:
            opened = stack.pop()
            if c != brackets[opened]:
                return 0
    
    # Complete and compute score
    score = 0
    while stack:
        score *= 5
        score += scores[stack.pop()]
        
    return score


with open('inputs.txt') as f:
    lines = f.read().splitlines()

scores = [line_score(l) for l in lines]
scores = sorted([s for s in scores if s != 0])

print(scores[int(len(scores) / 2)])