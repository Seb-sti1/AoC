from ast import literal_eval


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
    left = None
    right = None

    score = 0
    pair = 1

    for line in lines:
        if line.strip() != "":
            line = line.strip()

            if left is None:
                left = literal_eval(line)
            elif right is None:
                right = literal_eval(line)
                if process(left, right) == 1:
                    print(pair)
                    score += pair
        else:
            left = None
            right = None
            pair += 1

    print(score)


file = open('input', 'r')
iterate_line(file.readlines())
