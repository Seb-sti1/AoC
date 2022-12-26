from ast import literal_eval
from functools import cmp_to_key


def process(left, right):
    if isinstance(left, int) and isinstance(right, int):
        if left < right:
            return 1
        elif left > right:
            return -1
        else:
            return 0

    if isinstance(left, int):
        return process([left], right)

    if isinstance(right, int):
        return process(left, [right])

    for i in range(min(len(left), len(right))):
        leftEle = left[i]
        rightEle = right[i]

        p = process(leftEle, rightEle)
        if p in [1, -1]:
            return p

    if len(left) < len(right):
        return 1
    elif len(left) > len(right):
        return -1
    else:
        return 0


def iterate_line(lines):
    parsed_lines = [[[2]], [[6]]]

    for line in lines:
        if line.strip() != "":
            line = line.strip()

            parsed_lines.append(literal_eval(line))

    sorted_lines = sorted(parsed_lines, key=cmp_to_key(process), reverse=True)

    score = 1

    for i, line in enumerate(sorted_lines):
        print(line)
        if line == [[2]] or line == [[6]]:
            score *= (i + 1)
    print(score)


file = open('input', 'r')
iterate_line(file.readlines())
