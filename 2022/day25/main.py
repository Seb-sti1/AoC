def snafu_to_decimal(snafu):
    dec = 0

    for i, c in enumerate(snafu[::-1]):
        n = 0
        if c == "=":
            n = -2
        elif c == "-":
            n = -1
        else:
            n = int(c)

        dec += n * (5 ** i)

    return dec


def number_to_base(n, b):
    if n == 0:
        return [0]
    digits = []
    while n:
        digits.append(int(n % b))
        n //= b
    return digits


def decimal_to_snafu(n):
    base_5 = number_to_base(n, 5)
    base_5.append(0)

    for i, n in enumerate(base_5):
        if n >= 3:
            base_5[i + 1] += 1
            base_5[i] = - 5 + n

    if base_5[-1] == 0:
        base_5 = base_5[:-1]

    base_5 = base_5[::-1]
    base_snafu = []
    for n in base_5:
        if n >= 0:
            base_snafu.append(str(n))
        elif n == -1:
            base_snafu.append("-")
        elif n == -2:
            base_snafu.append("=")

    return ''.join(base_snafu)


def iterate_line(lines):
    sum_snafu = 0

    for i, line in enumerate(lines):
        if line.strip() != "":
            n = snafu_to_decimal(line.strip())
            print(f"{line.strip()} -> {n}")

            sum_snafu += n

    print(decimal_to_snafu(sum_snafu))


file = open('input', 'r')
iterate_line(file.readlines())
