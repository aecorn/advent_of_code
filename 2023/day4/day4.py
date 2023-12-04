with open("input.txt", "r") as inp_file:
    content = inp_file.read()
count_each_card = [1 for _ in content.split("\n")]
part1_sum = 0
for i, line in enumerate(content.split("\n")):
    remove_game = line.split(":")[1].strip()
    winning = [int(x) for x in remove_game.split("|")[0].strip().split(" ") if x]
    draws = [int(x) for x in remove_game.split("|")[1].strip().split(" ") if x]
    overlap = [x for x in draws if x in winning]
    for j in range(count_each_card[i]):
        for k in range(i+1, i+len(overlap)+1):
            #print(i, j, k)
            count_each_card[k] += 1
    score = 0
    if overlap:
        score = 1
        overlap.pop(0)
    for _ in overlap:
        score *= 2
    part1_sum += score
print("Part1:", part1_sum)
print("Part2:", sum(count_each_card))