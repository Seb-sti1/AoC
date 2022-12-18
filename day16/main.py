import re


class Node:
    def __init__(self, infos):
        self.name = infos[1]
        self.flow = int(infos[2])
        self.neighbors = {name: 1 for name in infos[3].split(', ')}

    def calculate_neighbors(self, tree):
        depth = 1
        neighbors, seen = [self.name], []
        while max([tree[self.name].neighbors[n] for n in tree if n != self.name]) == 1_000_000_000_000:

            neighbors, seen = get_neighbors_recursively(neighbors, tree, seen)
            for neighbor in neighbors:
                self.neighbors[neighbor] = min(self.neighbors[neighbor], depth, tree[neighbor].neighbors[self.name])
                tree[neighbor].neighbors[self.name] = self.neighbors[neighbor]

            depth += 1

    def remove_zero_neighbors(self, tree):
        neighbors = self.neighbors.copy()
        for n in neighbors:
            if tree[n].flow == 0:
                self.neighbors.pop(n)

    def __str__(self):
        return f"{self.name} [{self.flow}] : {', '.join([f'{n} : {self.neighbors[n]}' for n in self.neighbors])}"


def get_neighbors_recursively(border, tree, seen):
    new_border = []

    for node in border:
        seen += [node]
        new_border += [child for child in tree[node].neighbors if tree[node].neighbors[child] == 1
                       and child not in seen
                       and child not in new_border]

    return new_border, seen


paths = []


def max_pressure(n, tree, visited, time_left=30, score=0, save=False):
    if save and score > 1200:
        paths.append((score, visited))
    vls = []
    node = tree[n]

    for neighbor in node.neighbors:
        new_time_left = time_left - node.neighbors[neighbor] - 1
        if new_time_left >= 0 and neighbor not in visited:
            vls.append(max_pressure(neighbor,
                                    tree,
                                    visited + [neighbor],
                                    new_time_left,
                                    score + tree[neighbor].flow * new_time_left,
                                    save))

    if len(vls) == 0:
        return score
    return max(vls)


def iterate_line(lines):
    tree = {}

    for line in lines:
        if line.strip() != "":
            coords = re.search("^Valve ([A-Z][A-Z]) has flow rate=(\\d+); tunnels? leads? to valves? (([A-Z]+,? ?)+)$",
                               line.strip())

            tree[coords[1]] = Node(coords)

    for nA in tree:
        for nB in tree:
            if nA != nB and nB not in tree[nA].neighbors:
                tree[nA].neighbors[nB] = 1_000_000_000_000

    for n in tree:
        node = tree[n]
        node.calculate_neighbors(tree)

    for n in tree:
        node = tree[n]
        node.remove_zero_neighbors(tree)

    new_tree = tree.copy()
    for node in new_tree:
        if node != 'AA' and tree[node].flow == 0:
            tree.pop(node)
    #
    #
    #

    score = max_pressure('AA', tree, [])
    print(score)

    max_pressure('AA', tree, [], 26, 0, True)

    max_score = 0

    for i in range(len(paths)):
        for j in range(i, len(paths)):
            path = paths[i]
            p = paths[j]

            found = False
            for n in path[1]:
                if n in p[1]:
                    found = True
                    break

            if not found:
                if path[0] + p[0] > max_score:
                    max_score = path[0] + p[0]

    print(max_score)


file = open('input', 'r')
iterate_line(file.readlines())
