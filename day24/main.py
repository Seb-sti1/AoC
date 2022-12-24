class Blizzard:

    def __init__(self, orientation):
        self.orientation = orientation

    def __str__(self):
        if self.orientation == 0:
            return ">"
        elif self.orientation == 1:
            return "v"
        elif self.orientation == 2:
            return "<"
        elif self.orientation == 3:
            return "^"


class State:

    def __init__(self, i, j, n, m, blizzards, start, end, t=0):
        self.i = i
        self.j = j

        self.n = n
        self.m = m
        self.blizzards = blizzards
        self.start = start
        self.end = end

        self.t = t

    def next_states(self):
        possible_states = []
        next_blizzards = move_blizzards(self.n, self.m, self.blizzards)

        for a, b in [(-1, 0), (1, 0), (0, 0), (0, 1), (0, -1)]:
            new_i, new_j = (self.i + a, self.j + b)
            if ((0 < new_i < self.n - 1 and 0 < new_j < self.m - 1) or
                    (new_i, new_j) == self.start or
                    (new_i, new_j) == self.end)\
                    and (new_i, new_j) not in next_blizzards:
                possible_states.append(State(new_i, new_j,
                                             self.n, self.m, next_blizzards,
                                             self.start, self.end,
                                             self.t + 1))

        return possible_states

    def distance_to(self, point):
        return abs(point[0] - self.i) + abs(point[1] - self.j)

    def heuristique(self):
        return self.distance_to(self.end) + self.t

    def to_tuple(self):
        return self.i, self.j, self.t

    def __str__(self):
        return f"{self.i} {self.j} {self.t}"

    def __eq__(self, other):
        if isinstance(other, tuple):
            return self.to_tuple() == other
        return self.i == other.i and self.j == other.j and self.t == other.t

    def print(self):
        for i in range(self.n):
            for j in range(self.m):
                if self.i == i and self.j == j:
                    print("E", end="")
                elif self.start == (i, j):
                    print(".", end="")
                elif self.end == (i, j):
                    print(".", end="")
                elif i == 0 or i == self.n - 1 or j == 0 or j == self.m - 1:
                    print("#", end="")
                elif (i, j) in self.blizzards:
                    if len(self.blizzards[(i, j)]) > 1:
                        print(len(self.blizzards[(i, j)]), end="")
                    else:
                        o = self.blizzards[(i, j)][0].orientation
                        if o == 0:
                            print(">", end="")
                        elif o == 1:
                            print("v", end="")
                        elif o == 2:
                            print("<", end="")
                        elif o == 3:
                            print("^", end="")
                else:
                    print(".", end="")

            print()


def move_blizzards(n, m, blizzards):
    new_blizzards = {}

    for pos in blizzards:
        for B in blizzards[pos]:
            new_pos = None

            if B.orientation == 0:
                new_pos = (pos[0], pos[1] + 1)
            elif B.orientation == 1:
                new_pos = (pos[0] + 1, pos[1])
            elif B.orientation == 2:
                new_pos = (pos[0], pos[1] - 1)
            elif B.orientation == 3:
                new_pos = (pos[0] - 1, pos[1])

            if new_pos[0] == n - 1:
                new_pos = (1, new_pos[1])
            if new_pos[0] == 0:
                new_pos = (n - 2, new_pos[1])

            if new_pos[1] == m - 1:
                new_pos = (new_pos[0], 1)
            if new_pos[1] == 0:
                new_pos = (new_pos[0], m - 2)

            if new_pos in new_blizzards:
                new_blizzards[new_pos].append(B)
            else:
                new_blizzards[new_pos] = [B]

    return new_blizzards


def sorted_append(sorted_list, to_append):
    h = to_append.heuristique()
    for i in range(len(sorted_list)):
        if sorted_list[i].heuristique() > h:
            sorted_list.insert(i, to_append)
            return

    sorted_list.append(to_append)


def a_star(start, end):
    border = [start]
    seen = []

    iterations = 0

    while len(border) > 0:
        iterations += 1

        if iterations % 1_000 == 0:
            dist = [s.distance_to(end) for s in border]
            dist.sort()
            print(f"{len(border)} states in the border, {len(seen)} seen")
            print(f"Min dist is {min(dist)} max is {max(dist)} median is {dist[int(len(dist) / 2)]}")

        state = border.pop(0)

        if (state.i, state.j) == end:
            return state.t

        for i, next_state in enumerate(state.next_states()):
            if next_state not in seen and next_state not in border:
                sorted_append(border, next_state)

        seen.append(state.to_tuple())

    return -1


def iterate_line(lines):
    n, m = len(lines), -1
    blizzards = {}
    S = None
    E = None

    for i, line in enumerate(lines):
        if line.strip() != "":
            line = line.strip()
            m = len(line)
            for j in range(m):
                if i == 0 and line[j] == ".":
                    S = (i, j)
                if i == n - 1 and line[j] == ".":
                    E = (i, j)

                if line[j] == ">":
                    blizzards[(i, j)] = [Blizzard(0)]
                elif line[j] == "v":
                    blizzards[(i, j)] = [Blizzard(1)]
                elif line[j] == "<":
                    blizzards[(i, j)] = [Blizzard(2)]
                elif line[j] == "^":
                    blizzards[(i, j)] = [Blizzard(3)]

    score = a_star(State(S[0], S[1], n, m, blizzards, S, E), E)

    print(score)


file = open('input', 'r')
iterate_line(file.readlines())
