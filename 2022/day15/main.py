import re


def draw_scan(scan):
    for line in scan:
        for s in line:
            if s == 0:
                print(".", end="")
            elif s == 1:
                print("S", end="")
            elif s == 2:
                print("B", end="")
            elif s == 3:
                print("#", end="")
            else:
                print("?", end="")

        print()


def find_unknown_position(y, k, l, n, m, sensors, beacons):
    possible_x = [[k, m]]

    for S in sensors:

        dist_y = abs(S[1] - y)
        max_dist_x = S[2] - dist_y
        exclusion = [S[0] - max_dist_x, S[0] + max_dist_x]
        """
        S[0] - max_dist_x <= x <= S[0] + max_dist_x
        
        abs(x - S[0]) <= max_dist_x <=  S[2] - dist_y <= S[2] - abs(S[1] - y)
        abs(x - S[0]) + abs(S[1] - y) <= S[2]
        """

        if exclusion[0] <= exclusion[1]:
            new_possible_x = []

            for idx, inter in enumerate(possible_x):
                if exclusion[1] <= inter[0] or exclusion[0] >= inter[1]:
                    new_possible_x.append(inter)
                else:
                    if exclusion[0] > inter[0] and exclusion[1] < inter[1]:
                        new_possible_x.append([inter[0], exclusion[0] - 1])
                        new_possible_x.append([exclusion[1] + 1, inter[1]])

                    elif inter[0] < exclusion[1] < inter[1]:
                        inter[0] = exclusion[1] + 1

                        if inter[0] <= inter[1]:
                            new_possible_x.append(inter)
                    elif inter[1] > exclusion[0] > inter[0]:
                        inter[1] = exclusion[0] - 1

                        if inter[0] <= inter[1]:
                            new_possible_x.append(inter)

            possible_x = new_possible_x

            if len(possible_x) == 0:
                return False

    return possible_x[0][0], y


def iterate_line(lines):
    sensors = []
    beacons = []

    for line in lines:
        if line.strip() != "":
            coords = re.findall("[xy]=(-?\\d+)", line.strip())
            a, b = int(coords[0]), int(coords[1])
            x, y = int(coords[2]), int(coords[3])

            sensors.append((a, b, abs(a - x) + abs(b - y)))
            beacons.append((x, y))

    k, l = 0, 0
    n, m = 4000000 - 1, 4000000 - 1

    for y in range(m, l - 1, -1):
        result = find_unknown_position(y, k, l, n, m, sensors, beacons)

        if y % 1000 == 0:
            print(y * 100 / m)

        if result != False:
            print(result, result[0] * 4_000_000 + result[1])
            break


file = open('input', 'r')
iterate_line(file.readlines())
