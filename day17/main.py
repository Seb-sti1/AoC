import re
import time

pieces = [
    [(0, 0), (1, 0), (2, 0), (3, 0)],
    [(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
    [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
    [(0, 0), (0, 1), (0, 2), (0, 3)],
    [(0, 0), (0, 1), (1, 0), (1, 1)]
]


class Matrix:

    def __init__(self):
        self.pos = (0, 0)
        self.height = 4
        self.data = [[0 for j in range(7)] for i in range(4)]

        self.current_piece = -1
        self.highest = 0

    def refresh_highest_rock(self):
        self.highest = 0

        for i in range(self.highest, self.height):
            if sum(self.data[i]) == 0:
                self.highest = i
                break

    def draw(self):
        for i in range(self.height - 1, -1, -1):
            print("|", end="")
            for j in range(7):

                if self.data[i][j] == 0:
                    print(".", end="")
                elif self.data[i][j] == 1:
                    print("#", end="")
                elif self.data[i][j] == 2:
                    print("@", end="")
                else:
                    print("?", end="")

            print("|")
        print("+-------+")

    def add_height(self, to_add):
        to_add = max(to_add, 100)

        for i in range(to_add):
            self.data.append([0 for j in range(7)])

        self.height += to_add

    def spawn(self):
        self.current_piece = (self.current_piece + 1) % len(pieces)
        piece = pieces[self.current_piece]
        piece_height = max([y for x, y in piece]) + 1

        self.pos = 2, self.highest + 3

        to_add = self.highest + 3 + piece_height - self.height
        if to_add > 0:
            self.add_height(to_add)

    def end_move(self, piece):
        for position in piece:
            self.highest = max(self.highest, self.pos[1] + position[1] + 1)
            self.data[self.pos[1] + position[1]][self.pos[0] + position[0]] = 1


def iterate_line(lines):
    gaz = []

    for line in lines:
        if line.strip() != "":
            for dir in line:
                gaz.append(-1 if dir == "<" else 1)

    matrix = Matrix()

    rocks = 0
    gaz_idx = 0

    # 1_000_000_000_000
    N = 1_000_000_000_000
    T = max(int(N / 100), 1_000)

    percent = time.time()

    seen = []
    date = []

    period = None

    while rocks < N:
        matrix.spawn()
        piece = pieces[matrix.current_piece]
        rocks += 1

        state = (matrix.current_piece, gaz_idx)
        if state in seen:
            last_seen = date[seen.index(state) - 1]
            period = rocks - last_seen[0] - 1
            height_difference = matrix.highest - last_seen[1]

            if (N - rocks + 1) % period == 0:
                print(matrix.highest + (N - rocks + 1)//period*height_difference)
                break

        stopped = False
        pos = matrix.pos[0], matrix.pos[1]

        while not stopped:
            # push by gaz
            new_pos = pos[0] + gaz[gaz_idx], pos[1]
            gaz_idx = (gaz_idx + 1) % len(gaz)

            collide = False
            # check collisions
            for position in piece:
                if position[0] + new_pos[0] < 0 or position[0] + new_pos[0] >= 7 or \
                        matrix.data[new_pos[1] + position[1]][new_pos[0] + position[0]] == 1:
                    collide = True
                    break

            if not collide:
                pos = new_pos

            # move down
            new_pos = pos[0], pos[1] - 1

            stopped = False
            # check collisions
            for position in piece:
                if 0 > position[1] + new_pos[1] or matrix.data[new_pos[1] + position[1]][new_pos[0] + position[0]] == 1:
                    stopped = True
                    break

            if not stopped:
                pos = new_pos
            else:
                matrix.pos = pos

        matrix.end_move(piece)

        seen.append(state)
        date.append((rocks, matrix.highest))

        if rocks % T == 0:
            print(rocks / N * 100)
            print(time.time() - percent)

            percent = time.time()


file = open('input', 'r')
iterate_line(file.readlines())
