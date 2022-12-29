from compare_pair import compare_pair 

pairs: list[tuple[list, list]] = []
with open("./2022/day_13/input.txt", "r") as file:
    while True:
        first = eval(next(file).strip())
        second = eval(next(file).strip())
        pairs.append((first, second))

        last = next(file, None)
        if last == None:
            break

indicies = []
for i, pair in enumerate(pairs):
    first, second = pair
    if compare_pair(first, second) == 1:
        indicies.append(i + 1)

print(sum(indicies))
