from compare_pair import compare_pair
from copy import deepcopy

data: list[list] = []
with open("./2022/day_13/input.txt", "r") as file:
    for line in file:
        line = line.strip()
        if line != "":
            data.append(eval(line))

divider_packet_1 = [[2]]
divider_packet_2 = [[6]]

smaller_of_1 = 1
smaller_of_2 = 2

for line in data:
    if compare_pair(deepcopy(line), deepcopy(divider_packet_1)) == 1:
        smaller_of_1 += 1
    if compare_pair(deepcopy(line), deepcopy(divider_packet_2)) == 1:
        smaller_of_2 += 1

print(smaller_of_1, smaller_of_2)
print(smaller_of_1 * smaller_of_2)
