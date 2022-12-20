class Number:
    def __init__(self, n):
        self.n = n
        self.moved = False

    def __str__(self):
        if self.moved:
            return f"({self.n})"
        return f"{self.n}"


def mixing(numbers):
    print(len(numbers))
    moved = 0
    while moved < len(numbers):

        if moved % 1000 == 0:
            print(moved / len(numbers) * 100)
        to_moved = -1

        for idx in range(len(numbers)):
            if not numbers[idx].moved:
                to_moved = idx
                break

        if to_moved == -1:
            break

        N = numbers.pop(to_moved)
        N.moved = True
        numbers.insert((to_moved + N.n) % len(numbers), N)

        moved += 1


def show_list(numbers):
    print(", ".join([str(N) for N in numbers]))


def iterate_line(lines):
    numbers = []

    for line in lines:
        if line.strip() != "":
            i = int(line.strip())

            numbers.append(Number(i))

    mixing(numbers)

    # Find the zero
    position_of_zero = -1
    for i in range(len(numbers)):
        if numbers[i].n == 0:
            position_of_zero = i
            break

    # Look for key position
    numbers_to_look = [1000, 2000, 3000]
    numbers_found = [-1, -1, -1]

    for i, number_to_look in enumerate(numbers_to_look):
        pos = position_of_zero + number_to_look
        pos = pos % len(numbers)
        numbers_found[i] = numbers[pos].n

    # print out the score
    print(numbers_found)
    print(sum(numbers_found))


file = open('input', 'r')
iterate_line(file.readlines())
