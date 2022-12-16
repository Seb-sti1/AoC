import re


def search_tree(node, tree, seen, time_left):
    if len(seen) == len(tree) - 1 or time_left == 0:
        return 0

    score = time_left * tree[node]["flow"]
    max_total = 0

    for neighbor in tree[node]["neighbors"]:
        if neighbor not in seen:
            time_to_spend = tree[node]["neighbors"][neighbor] + 1

            if time_to_spend <= time_left:
                max_total = max(max_total, search_tree(neighbor,
                                                       tree,
                                                       seen + [node],
                                                       time_left - time_to_spend))

    return score + max_total


def get_neighbors_recursively(border, tree, seen):
    new_border = []

    for node in border:
        seen += [node]
        new_border += [child for child in tree[node]["neighbors"] if tree[node]["neighbors"][child] == 1
                       and child not in seen
                       and child not in new_border]

    return new_border, seen


def iterate_line(lines):
    tree = {}
    to_open = 0

    for line in lines:
        if line.strip() != "":
            coords = re.search("^Valve ([A-Z][A-Z]) has flow rate=(\\d+); tunnels? leads? to valves? (([A-Z]+,? ?)+)$",
                               line.strip())

            if int(coords[2]) > 0:
                to_open += 1

            tree[coords[1]] = {"flow": int(coords[2]),
                               "neighbors": {name: 1 for name in coords[3].split(', ')}}

    for node in tree:
        for n in tree:
            if node != n and n not in tree[node]["neighbors"]:
                tree[node]["neighbors"][n] = -1

    for node in tree:
        depth = 1
        neighbors, seen = [node], []
        while min([tree[node]["neighbors"][n] for n in tree if n != node]) == -1:

            neighbors, seen = get_neighbors_recursively(neighbors, tree, seen)
            for neighbor in neighbors:
                tree[node]["neighbors"][neighbor] = depth
                tree[neighbor]["neighbors"][node] = depth

            depth += 1

    for node in tree:
        for n in tree:
            if node != n and tree[n]["flow"] == 0:
                tree[node]["neighbors"].pop(n)

    new_tree = tree.copy()
    for node in tree:
        if node != 'AA' and tree[node]["flow"] == 0:
            new_tree.pop(node)
    tree = new_tree

    print(search_tree('AA', tree, [], 30))


file = open('input', 'r')
iterate_line(file.readlines())
