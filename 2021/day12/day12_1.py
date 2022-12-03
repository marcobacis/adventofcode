
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
    
def visit(node, end, path):
    #if not node.isupper() and node != "end":
    #    visited.add(node)

    #print("Visiting " + node)

    if node == "end":
        #print(" -> ".join(path + ["end"]))
        return 1
    else:
        to_append = [n for n in adj[node] if n not in path or n.isupper()]
        #print(to_append)
        return sum([visit(n, end, path + [node]) for n in to_append])
    
    return 0

visits = visit("start", "end", [])
print(visits)