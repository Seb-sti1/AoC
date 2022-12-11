class Monkey:
    inspected_items = 0
    items = []

    def __init__(self):
        self.false_monkey = None
        self.true_monkey = None
        self.test = None
        self.operation = None

    def set_starting_items(self, starting_items):
        print(starting_items)
        self.items = starting_items

    def set_operation(self, operation):
        print(operation)
        self.operation = operation

    def set_test(self, test):
        print(test)
        self.test = test

    def set_true_monkey(self, true_monkey):
        print(true_monkey)
        self.true_monkey = true_monkey

    def set_false_monkey(self, false_monkey):
        print(false_monkey)
        self.false_monkey = false_monkey

    def inspect_item(self):
        if len(self.items) == 0:
            return None
        self.inspected_items += 1

        item = self.items.pop(0)
        print(f"{item} -> {self.operation.replace('old', str(item))} -> ", end="")
        item = eval(self.operation.replace('old', str(item)))
        item = int(item / 3)
        print(item)

        if item % self.test == 0:
            return self.true_monkey, item
        else:
            return self.false_monkey, item


monkeys = []


def iterate_line(lines):
    global monkeys

    x = 1
    cycle = 0

    monkey = None

    for line in lines:
        if line.strip() != "":
            words = line.strip().split(" ")

            if words[0] == "Monkey":
                monkey = Monkey()
            elif words[0] == "Starting":
                monkey.set_starting_items([int(words[i].replace(',', '')) for i in range(2, len(words))])
            elif words[0] == "Operation:":
                monkey.set_operation(" ".join(words[3:]))
            elif words[0] == "Test:":
                monkey.set_test(int(words[3]))
            elif words[0] == "If" and words[1] == "true:":
                monkey.set_true_monkey(int(words[5]))
            elif words[0] == "If" and words[1] == "false:":
                monkey.set_false_monkey(int(words[5]))
                monkeys.append(monkey)


file = open('input', 'r')
iterate_line(file.readlines())

for i in range(20):  # round
    print(f"Round {i + 1}")
    for monkey in monkeys:
        for j in range(len(monkey.items)):
            result = monkey.inspect_item()
            print(result)

            if result is not None:
                idx, item = result
                monkeys[idx].items.append(item)

    print(f"After round {i+1}")
    j = 0
    for monkey in monkeys:
        print(f"Monkey {j} : {' '.join([str(e) for e in monkey.items])}")
        j += 1

for monkey in monkeys:
    print(f"Monkey {j} inspected items {monkey.inspected_items} times")
