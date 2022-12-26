import re


class Blueprint:
    def __init__(self, id, ore_robot, clay_robot, obsidian_robot, geode_robot):
        self.id = id

        self.robot_recipes = []

        self.robot_recipes.append(ore_robot)
        self.robot_recipes.append(clay_robot)
        self.robot_recipes.append(obsidian_robot)
        self.robot_recipes.append(geode_robot)

        self.max_conso = None

    def space_size(self, max_geodes):
        return (self.max_conso.ore * self.max_conso.clay * self.max_conso.obsidian * max(max_geodes, 5)) ** 2


class Materials:
    def __init__(self, ore=0, clay=0, obsidian=0, geode=0):
        self.ore = ore
        self.clay = clay
        self.obsidian = obsidian
        self.geode = geode

    def copy(self):
        return Materials(self.ore, self.clay, self.obsidian, self.geode)


class Robots:
    def __init__(self, ore, clay, obsidian, geode):
        self.ore = ore
        self.clay = clay
        self.obsidian = obsidian
        self.geode = geode

    def add(self, t):
        if t == 0:
            self.ore += 1
        elif t == 1:
            self.clay += 1
        elif t == 2:
            self.obsidian += 1
        elif t == 3:
            self.geode += 1

    def copy(self):
        return Robots(self.ore, self.clay, self.obsidian, self.geode)


class State:
    def __init__(self, robots, materials, time_left):
        self.robots = robots
        self.materials = materials
        self.time_left = time_left

    def next_states(self, blueprint):
        acceptable_states = []

        max_conso = blueprint.max_conso

        for i in range(len(blueprint.robot_recipes) - 1, -1, -1):
            recipe = blueprint.robot_recipes[i]

            if i == 0 and self.robots.ore >= max_conso.ore:
                continue

            if i == 1 and self.robots.clay >= max_conso.clay:
                continue

            if i == 2 and self.robots.obsidian >= max_conso.obsidian:
                continue

            if recipe[0] <= self.materials.ore and recipe[1] <= self.materials.clay and \
                    recipe[2] <= self.materials.obsidian:
                new_materials = self.materials.copy()
                new_materials.ore += self.robots.ore - recipe[0]
                new_materials.clay += self.robots.clay - recipe[1]
                new_materials.obsidian += self.robots.obsidian - recipe[2]
                new_materials.geode += self.robots.geode

                new_robots = self.robots.copy()
                new_robots.add(i)

                acceptable_states.append(State(new_robots, new_materials, self.time_left - 1))

        if (self.materials.ore < max_conso.ore and self.robots.ore > 0) \
                or (self.materials.clay < max_conso.clay and self.robots.clay > 0) \
                or (self.materials.obsidian < max_conso.obsidian and self.robots.obsidian > 0) \
                or self.robots.geode > 0:
            new_materials = self.materials.copy()
            new_materials.ore += self.robots.ore
            new_materials.clay += self.robots.clay
            new_materials.obsidian += self.robots.obsidian
            new_materials.geode += self.robots.geode

            acceptable_states.append(State(self.robots.copy(), new_materials, self.time_left - 1))

        return acceptable_states

    def theoretical_max(self):
        """

        :return:
        """

        """
        max_geodes = self.materials.geode

        j = 0
        for i in range(self.time_left, -1, -1):
            max_geodes += (self.robots.geode + j) * i
            j += 1
        """

        return self.time_left * (self.time_left + 1) * (3 * self.robots.geode + self.time_left - 1) / 6 \
            + self.materials.geode


def dfs(state, blueprint, max_geodes, recur=0):
    if state.time_left <= 0:
        return max(state.materials.geode, max_geodes)

    for new_state in state.next_states(blueprint):
        if max_geodes < 0 or new_state.theoretical_max() > max_geodes:
            max_geodes = max(dfs(new_state, blueprint, max_geodes, recur + 1), max_geodes)

    return max_geodes


def iterate_line(lines):
    blueprints = []

    for line in lines:
        if line.strip() != "":
            b = re.search(
                r"^Blueprint (\d+): Each ore robot costs (\d+) ore\. Each clay robot costs (\d+) ore\. Each ob"
                r"sidian robot costs (\d+) ore and (\d+) clay\. Each geode robot costs (\d+) ore and (\d+) obsidian\.$",
                line.strip())

            blueprints.append(Blueprint(int(b[1]),
                                        (int(b[2]), 0, 0),
                                        (int(b[3]), 0, 0),
                                        (int(b[4]), int(b[5]), 0),
                                        (int(b[6]), 0, int(b[7]))
                                        ))

    score = 1
    for blueprint in blueprints[:3]:
        max_conso = Materials()

        for recipe in blueprint.robot_recipes:
            max_conso.ore = max(max_conso.ore, recipe[0])
            max_conso.clay = max(max_conso.clay, recipe[1])
            max_conso.obsidian = max(max_conso.obsidian, recipe[2])

        blueprint.max_conso = max_conso

        max_geodes = dfs(State(Robots(1, 0, 0, 0), Materials(), 32), blueprint, 0)
        print(f"Max geodes for blueprint {blueprint.id} is {max_geodes}")

        score *= max_geodes

    print(score)


file = open('input', 'r')
iterate_line(file.readlines())
