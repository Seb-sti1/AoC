class Monkey:
    def __init__(self, name, operation_s):
        self.name = name
        self.operation_s = operation_s

        self.value = -1
        self.necessary_monkeys = []

        words = self.operation_s.split(" ")
        if len(words) > 1:
            self.necessary_monkeys.append(words[0])
            self.necessary_monkeys.append(words[2])
        else:
            self.value = int(words[0])

    def __str__(self):
        if self.value == -1:
            return f"{self.name}: {self.operation_s}"
        return f"{self.name}: {self.value}"


def dict_and(dict):
    AND = True
    for key in dict:
        AND = AND and dict[key]

        if not AND:
            return False

    return AND


def all_necessary_monkeys(search_list, necessary_monkeys):
    # create a dict
    found = {}
    for necessary_monkey in necessary_monkeys:
        found[necessary_monkey] = False

    # search for necessary monkeys
    for monkey in search_list:
        if monkey.name in found:
            found[monkey.name] = True

            if dict_and(found):
                return True

    return dict_and(found)


def sorted_fusion(sorted_list, to_add):
    if len(to_add) == 0:
        return sorted_list

    new_sorted_list = sorted_list + []
    new_to_add = []

    for monkey in to_add:

        if all_necessary_monkeys(new_sorted_list, monkey.necessary_monkeys):
            new_sorted_list.append(monkey)
        else:
            new_to_add.append(monkey)

    return sorted_fusion(new_sorted_list, new_to_add)


def eval_monkeys(sorted_list):

    values = {}

    for M in sorted_list:
        if M.value == -1:
            operation = M.operation_s

            for necessary_monkey in M.necessary_monkeys:
                operation = operation.replace(necessary_monkey, str(values[necessary_monkey]))

            M.value = int(eval(operation))

        values[M.name] = M.value

    return values

def iterate_line(lines):
    monkeys = []

    cant_eval_monkeys = []

    for i, line in enumerate(lines):
        if line.strip() != "":
            name, operation_s = line.strip().split(": ")

            M = Monkey(name, operation_s)
            if len(M.necessary_monkeys) == 0:
                monkeys.append(M)
            else:
                if all_necessary_monkeys(monkeys, M.necessary_monkeys):
                    monkeys.append(M)
                else:
                    cant_eval_monkeys.append(M)

    monkeys_sorted = sorted_fusion(monkeys, cant_eval_monkeys)

    values = eval_monkeys(monkeys_sorted)

    print(values["root"])


file = open('input', 'r')
iterate_line(file.readlines())
