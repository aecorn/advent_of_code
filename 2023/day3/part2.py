with open("input.txt", "r") as inp_file:
    content = inp_file.read()
content_split = [x for x in content.split("\n")]
#print(content_split)
gear_values = {}
for y, line in enumerate(content.split("\n")):
    curr_num = ""
    switch = False
    pos = {}
    for x, c in enumerate(line):
        if c.isnumeric() and x != len(line)-1:
            curr_num += c
            switch = True
        elif switch or x == len(line)-1:
            if x == len(line)-1 and c.isnumeric():
                curr_num += c
                x += 1
            # End of number
            x_range = range(x-len(curr_num), x+1)
            top = [(x_rn, y-1) for x_rn in range(x-len(curr_num)-1, x+1) ]
            sides = [(x-len(curr_num)-1, y), (x, y)]
            bottom = [(x_rn, y+1) for x_rn in range(x-len(curr_num)-1, x+1) ]
            border = [xy for xy in top + sides + bottom if xy[0] in range(140) and xy[1] in range(140)]
            #if curr_num == "649":
            #    print("printing 649", curr_num, border)
            #print(border)
            for pos in border:
                #print(pos)
                if content_split[pos[1]][pos[0]] == "*":
                    #if curr_num == "649":
                    #    print(curr_num, pos, border)
                    if 80 < pos[1] < 86:
                        #pass
                        print(curr_num, pos, border)
                    #print(pos, curr_num)
                    if pos in gear_values:
                        gear_values[pos] += [curr_num]
                    else:
                        gear_values[pos] = [curr_num]
            curr_num = ""
            switch = False
part2_sum = 0
for pos, values in gear_values.items():
    if len(values) == 2:
        part2_sum += int(values[0]) * int(values[1])
print(part2_sum)
#print(gear_values)