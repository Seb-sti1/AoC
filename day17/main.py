import re

pieces = [
    [(0, 0), (1, 0), (2, 0), (3, 0)],
    [(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
    [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
    [(0, 0), (0, 1), (0, 2), (0, 3)],
    [(0, 0), (0, 1), (1, 0), (1, 1)]
]


class Matrix:

    def __init__(self):
        self.pos = None
        self.height = 4
        self.data = [[0 for j in range(7)] for i in range(4)]

        self.current_piece = -1

        self.removed_lines = 0

    def get_highest_rock(self):
        highest = 0

        for i in range(self.height):
            if sum(self.data[i]) == 0:
                highest = i
                break

        return highest

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
        for i in range(to_add):
            self.data.append([0 for j in range(7)])

        self.height += to_add

    def spawn(self):
        self.current_piece = (self.current_piece + 1) % len(pieces)
        piece = pieces[self.current_piece]
        piece_height = max([y for x, y in piece]) + 1

        spawn_height = self.get_highest_rock() + 3
        self.pos = 2, spawn_height

        to_append = spawn_height + piece_height - self.height
        self.add_height(to_append)

        for position in piece:
            self.data[self.pos[1] + position[1]][self.pos[0] + position[0]] = 2

    def move(self, x, y):
        new_pos = self.pos[0] + x, self.pos[1] + y

        piece = pieces[self.current_piece]

        # check collisions
        for position in piece:
            if 0 > position[1] + new_pos[1] or position[0] + new_pos[0] < 0 or position[0] + new_pos[0] >= 7:
                return False
            elif self.data[new_pos[1] + position[1]][new_pos[0] + position[0]] == 1:
                return False

        # move
        for position in piece:
            self.data[self.pos[1] + position[1]][self.pos[0] + position[0]] = 0

        for position in piece:
            self.data[new_pos[1] + position[1]][new_pos[0] + position[0]] = 2

        self.pos = new_pos

        return True

    def end_move(self):
        piece = pieces[self.current_piece]

        for position in piece:
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

    while rocks < 2022:

        matrix.spawn()
        rocks += 1

        stopped = False
        while not stopped:
            # push by gaz
            matrix.move(gaz[gaz_idx], 0)
            gaz_idx = (gaz_idx + 1) % len(gaz)

            # move down
            stopped = not matrix.move(0, -1)

        matrix.end_move()
    matrix.draw()
    print(matrix.get_highest_rock())


file = open('input', 'r')
iterate_line(file.readlines())
