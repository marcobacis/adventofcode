
from collections import Counter

with open('inputs.txt','r') as f:
    paths = [l.split('-') for l in f.read().splitlines()]

nodes = set([p for (p,_) in paths] + [p for (_, p) in paths])
adj = {}
for (s,d) in paths:
    if s in adj:
        adj[s].add(d)
    else:
        adj[s] = set([d])
    
    if d in adj:
        adj[d].add(s)
    else:
        adj[d] = set([s])

def visit(node, path):
    counts = Counter([n for n in path if n.islower() and n != "start"])
    if len([1 for (k,v) in counts.items() if v == 2]) > 1:
        return 0

    if node == "end":
        #print(" -> ".join(path + ["end"]))
        return 1
    
    to_append = [n for n in adj[node] if n not in path or (counts[n] == 1) or n.isupper()]
    return sum([visit(n, path + [node]) for n in to_append])
    
visits = visit("start", {})
print(visits)