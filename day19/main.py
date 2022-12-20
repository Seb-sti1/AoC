import re


class Blueprint:
    def __init__(self, id, ore_robot, clay_robot, obsidian_robot, geode_robot):
        self.id = id

        self.robot_recipes = []

        self.robot_recipes.append(ore_robot)
        self.robot_recipes.append(clay_robot)
        self.robot_recipes.append(obsidian_robot)
        self.robot_recipes.append(geode_robot)


class Materials:
    def __init__(self):
        self.ore = 0
        self.clay = 0
        self.obsidian = 0
        self.geode = 0

    def copy(self):
        n = Materials()
        n.ore = self.ore
        n.clay = self.clay
        n.obsidian = self.obsidian
        n.geode = self.geode

        return n

    def __str__(self):
        return f"ore {self.ore} clay {self.clay} obsidian {self.obsidian} geode {self.geode}"


def max_geodes(robots, materials, max_conso, blueprint, time_left=24):
    if time_left == 0:
        return materials.geode

    vls = []

    # No robot created, all mine
    new_materials = materials.copy()
    new_materials.ore += robots[0]
    new_materials.clay += robots[1]
    new_materials.obsidian += robots[2]
    new_materials.geode += robots[3]

    if (new_materials.ore <= max_conso.ore and robots[0] > 0)\
            or (new_materials.clay <= max_conso.clay and robots[1] > 0)\
            or (new_materials.obsidian <= max_conso.obsidian and robots[2] > 0):
        vls.append(max_geodes(robots, new_materials, max_conso, blueprint, time_left - 1))

    for i, recipe in enumerate(blueprint.robot_recipes):
        if i == 0 and robots[0] >= max_conso.ore:
            continue

        if i == 1 and robots[1] >= max_conso.clay:
            continue

        if i == 2 and robots[2] >= max_conso.obsidian:
            continue

        if recipe[0] <= materials.ore and recipe[1] <= materials.clay and recipe[2] <= materials.obsidian:
            new_materials_with_new_robot = new_materials.copy()
            new_materials_with_new_robot.ore -= recipe[0]
            new_materials_with_new_robot.clay -= recipe[1]
            new_materials_with_new_robot.obsidian -= recipe[2]

            new_robots = robots.copy()
            new_robots[i] += 1

            vls.append(max_geodes(new_robots, new_materials_with_new_robot, max_conso, blueprint, time_left - 1))

    if len(vls) == 0:
        return materials.geode
    return max(vls)


def iterate_line(lines):
    blueprints = []

    for line in lines:
        if line.strip() != "":
            b = re.search(r"^Blueprint (\d+): Each ore robot costs (\d+) ore\. Each clay robot costs (\d+) ore\. Each obsidian robot costs (\d+) ore and (\d+) clay\. Each geode robot costs (\d+) ore and (\d+) obsidian\.$",
                          line.strip())

            blueprints.append(Blueprint(int(b[1]),
                                        (int(b[2]), 0, 0),
                                        (int(b[3]), 0, 0),
                                        (int(b[4]), int(b[5]), 0),
                                        (int(b[6]), 0, int(b[7]))
                                        ))

    score = 0
    for blueprint in blueprints:
        max_conso = Materials()

        for recipe in blueprint.robot_recipes:
            max_conso.ore = max(max_conso.ore, recipe[0])
            max_conso.clay = max(max_conso.clay, recipe[1])
            max_conso.obsidian = max(max_conso.obsidian, recipe[2])

        score += blueprint.id * max_geodes([1, 0, 0, 0], Materials(), max_conso, blueprint)

    print(score)


file = open('test_input', 'r')
iterate_line(file.readlines())
