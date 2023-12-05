with open("small_input.txt", "r") as inp_file:
    content = inp_file.read()
parts = content.split("\n\n")
seeds = [int(x) for x in parts[0].split(":")[1].strip().split(" ")]
steps = [[[int(z) for z in x.split(" ")] for x in parts[y].split("\n")[1:]] for y in range(1, 8)]
for step in steps:
    mapping = {}
    for row in step:
        diff = row[1] - row[0]
        mapping[range(row[1], row[1]+row[2])] = diff
    translation = []
    for seed in seeds:
        for k, v in mapping.items():
            if seed in k:
                translation += [seed - v]
                break
        else:
            translation += [seed]
    seeds = translation
    print(seeds)
print(min(seeds))