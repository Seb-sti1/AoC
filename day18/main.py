import re
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
import numpy as np


class Cube:
    def __init__(self, x, y, z):
        self.x, self.y, self.z = x, y, z

    def get_neighbors(self):

        neighbors = []
        for i in range(-1, 2, 1):
            if i != 0:
                neighbors.append((self.x + i, self.y, self.z))

        for i in range(-1, 2, 1):
            if i != 0:
                neighbors.append((self.x, self.y + i, self.z))

        for i in range(-1, 2, 1):
            if i != 0:
                neighbors.append((self.x, self.y, self.z + i))

        return neighbors

    def to_tuple(self):
        return self.x, self.y, self.z


def iterate_line(lines):
    cubes = []

    for line in lines:
        if line.strip() != "":
            x, y, z = line.strip().split(",")

            cubes.append(Cube(int(x), int(y), int(z)))

    score = 0
    cubes_tuples = [c.to_tuple() for c in cubes]

    for cube in cubes:
        for neighbor in cube.get_neighbors():
            if neighbor not in cubes_tuples:
                score += 1

    print(score)


file = open('test_input', 'r')
iterate_line(file.readlines())
