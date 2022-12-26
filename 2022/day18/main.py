import re
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
import numpy as np


def get_neighbors(cube, axes=None, in_range=False):
    x, y, z = cube

    neighbors = []
    for i in range(-1, 2, 1):
        if i != 0 and (not in_range or axes[0] > x + i >= 0):
            neighbors.append((x + i, y, z))

    for i in range(-1, 2, 1):
        if i != 0 and (not in_range or axes[1] > y + i >= 0):
            neighbors.append((x, y + i, z))

    for i in range(-1, 2, 1):
        if i != 0 and (not in_range or axes[2] > z + i >= 0):
            neighbors.append((x, y, z + i))

    return neighbors


def fill_with_water(cubes, axes, data):
    next_cubes = []

    if len(cubes) == 0:
        return

    for cube in cubes:
        data[cube] = (0, 0, 1, 0.2)

        for neighbor in get_neighbors(cube, axes, in_range=True):
            if sum(data[neighbor]) == 0 and neighbor not in next_cubes:
                next_cubes.append(neighbor)

    fill_with_water(next_cubes, axes, data)


def nb_face_to_water(cube, data):
    score = 0

    neighbors = get_neighbors(cube, data.shape, in_range=True)

    if len(neighbors) < 6:
        score += 6 - len(neighbors)

    for neighbor in neighbors:
        if data[neighbor][2] == 1:
            score += 1

    return score


def next_to_water(cube, data):
    for neighbor in get_neighbors(cube, data.shape, in_range=True):
        if data[neighbor][2] == 1:
            return True
    return False


def iterate_line(lines):
    axes = [22, 22, 22]
    data = np.zeros(axes + [4], dtype=np.float32)

    for line in lines:
        if line.strip() != "":
            x, y, z = line.strip().split(",")
            x, y, z = int(x), int(y), int(z)
            data[x, y, z] = (1, 0, 0, 0.2)

    fill_with_water([(0, 0, 0)], axes, data)

    # plot
    plot_data = np.zeros(axes, dtype=bool)

    score = 0

    for x in range(axes[0]):
        for y in range(axes[1]):
            for z in range(axes[2]):
                if data[x, y, z, 0] == 1:  # it's lava
                    s = nb_face_to_water((x, y, z), data)
                    score += s
                    plot_data[x, y, z] = s > 0

    print(score)

    fig = plt.figure()
    ax = fig.add_subplot(111, projection='3d')

    # Voxels are used to customizations of the
    # sizes, positions and colors.
    ax.voxels(plot_data, facecolors=data)
    plt.show()


file = open('input', 'r')
iterate_line(file.readlines())
