def get_neighbors(point, n, m, height_map, seen):
    neighbors = []
    possible_neighbors = [(point[0] - 1, point[1]),
                          (point[0] + 1, point[1]),
                          (point[0], point[1] - 1),
                          (point[0], point[1] + 1)]

    for possible_neighbor in possible_neighbors:
        i, j = possible_neighbor
        if n > i >= 0 and m > j >= 0 and possible_neighbor not in seen:
            if 0 <= height_map[i][j] <= height_map[point[0]][point[1]] + 1:
                neighbors.append((i, j))

    return neighbors


def width_search(to_search, n, m, height_map, seen, goal):
    next_to_search = []
    next_seen = seen + []

    for node in to_search:
        point, dist = node

        if point == goal:
            return dist
        else:
            next_seen.append(point)

            for next_point in get_neighbors(point, n, m, height_map, seen):
                idx = -1
                for i in range(len(next_to_search)):
                    if next_to_search[i][0] == next_point:
                        idx = i
                        break

                if idx == -1:
                    next_to_search.append((next_point, dist + 1))
                elif next_to_search[idx][1] > dist + 1:
                    next_to_search.pop(idx)
                    next_to_search.append((next_point, dist + 1))

    """
    for i in range(len(height_map)):
        for j in range(len(height_map[0])):
            if (i, j) == goal:
                print("G", end="")
            elif (i, j) in [node[0] for node in next_to_search]:
                print("F", end="")
            elif (i, j) in next_seen:
                print("_", end="")
            else:
                print("?", end="")
        print()

    print()
    print()
    """

    if len(next_to_search) == 0:
        return -1
    else:
        return width_search(next_to_search, n, m, height_map, next_seen, goal)


def find_path(n, m, height_map, start, goal):
    to_search = [(start, 0)]
    dist = width_search(to_search, n, m, height_map, [], goal)
    return dist


def iterate_line(lines):
    height_map = []
    start = None
    goal = None

    i = 0
    for line in lines:
        if line.strip() != "":
            height_map.append([])
            j = 0
            for char in line:
                if char == "S":
                    start = (i, j)
                    height_map[i].append(0)
                    j += 1
                elif char == "E":
                    goal = (i, j)
                    height_map[i].append(25)
                    j += 1
                elif 26 > ord(char) - ord("a") >= 0:
                    height_map[i].append(ord(char) - ord("a"))
                    j += 1

            i += 1

    n, m = len(height_map), len(height_map[0])

    nb_a = 0
    for i in range(n):
        for j in range(m):
            if height_map[i][j] == 0:
                nb_a += 1
    print(nb_a)

    cand = 1000
    tested = 0

    for i in range(n):
        for j in range(m):
            if height_map[i][j] == 0:
                dist = find_path(n, m, height_map, (i, j), goal)
                tested += 1
                if dist != -1 and dist < cand:
                    print(f"{tested/nb_a*100} % : {dist} min {cand}")
                    cand = dist

    print(cand)


file = open('input', 'r')
iterate_line(file.readlines())
