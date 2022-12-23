import re


def find_duplicates(array):
    uniques = []
    duplicates = []

    for ele in array:
        if ele not in uniques:
            uniques.append(ele)
        elif ele not in duplicates:
            duplicates.append(ele)

    return duplicates


def sum_tuple(t, u):
    return t[0] + u[0], t[1] + u[1]


def no_elves(elves, pos, relative_positions):
    for to_check in relative_positions:

        if sum_tuple(pos, to_check) in elves:
            return False

    return True


class Elves:
    def __init__(self, pos):
        self.pos = pos
        self.new_pos = None
        self.idx_proposition = -1
        self.propositions = [('N', (-1, -1), (-1, 0), (-1, 1)),
                             ('S', (1, -1), (1, 0), (1, 1)),
                             ('W', (1, -1), (0, -1), (-1, -1)),
                             ('E', (1, 1), (0, 1), (-1, 1))]

    def search_proposition(self, elves):
        N = len(self.propositions)
        self.idx_proposition = (self.idx_proposition + 1) % N

        if no_elves(elves, self.pos, [(-1, -1), (-1, 0), (-1, 1), (1, -1), (1, 0), (1, 1), (0, 1), (0, -1)]):
            return None

        acceptable_proposition = None

        for i in range(N):
            proposition = self.propositions[(i + self.idx_proposition) % N]

            if no_elves(elves, self.pos, proposition[1:]):
                acceptable_proposition = proposition
                break

        if acceptable_proposition is not None:
            if acceptable_proposition[0] == 'N':
                self.new_pos = (self.pos[0] - 1, self.pos[1])
            elif acceptable_proposition[0] == 'S':
                self.new_pos = (self.pos[0] + 1, self.pos[1])
            elif acceptable_proposition[0] == 'E':
                self.new_pos = (self.pos[0], self.pos[1] + 1)
            elif acceptable_proposition[0] == 'W':
                self.new_pos = (self.pos[0], self.pos[1] - 1)

            return self.new_pos

        return None

    def __str__(self):
        return f"Elves {self.pos} -> {self.new_pos}"


def print_elves(elves):
    i_min, i_max = min([pos[0] for pos in elves]), max([pos[0] for pos in elves])
    j_min, j_max = min([pos[1] for pos in elves]), max([pos[1] for pos in elves])

    print(f"{i_min}, {j_min}")
    score = 0

    for i in range(i_min, i_max + 1):
        for j in range(j_min, j_max + 1):
            if (i, j) in elves:
                print("#", end="")
            else:
                print(".", end="")
                score += 1

        print()

    return score


def iterate_line(lines):
    elves = {}

    for i, line in enumerate(lines):
        if line.strip() != "":
            for j in range(len(line.strip())):
                if line[j] == "#":
                    elves[(i, j)] = Elves((i, j))

    print_elves(elves)
    print()

    score = 0
    i = 0
    still_moving = True
    while still_moving and i < 10:
        i += 1
        new_positions = []
        for pos in elves:
            e = elves[pos]

            new_pos = e.search_proposition(elves)
            if new_pos is not None:
                new_positions.append(new_pos)

        if len(new_positions) == 0:
            still_moving = False
            break

        duplicates = find_duplicates(new_positions)
        new_elves = {}

        for pos in elves:
            e = elves[pos]

            if e.new_pos in duplicates:
                e.new_pos = None
            else:
                if e.new_pos is not None:
                    e.pos = e.new_pos
                    e.new_pos = None

            new_elves[e.pos] = e
        elves = new_elves

        print(f"Round {i}")
        score = print_elves(elves)
        print()

    print(f"The score after 10 round is {score}")


file = open('input', 'r')
iterate_line(file.readlines())
