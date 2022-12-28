def compare_pair(first: list, second: list) -> int:
    while len(first) > 0 and len(second) > 0:
        left = first.pop(0)
        right = second.pop(0)

        if type(left) == int and type(right) == int:
            if left < right:
                return 1
            elif left > right:
                return -1
            continue

        sub_comparison = 0

        if type(left) == list and type(right) == list:
            sub_comparison = compare_pair(left, right)
        elif type(left) == int and type(right) == list:
            sub_comparison = compare_pair([left], right)
        elif type(left) == list and type(right) == int:
            sub_comparison = compare_pair(left, [right])

        if sub_comparison != 0:
            return sub_comparison

    if len(first) < len(second):
        return 1
    elif len(first) > len(second):
        return -1

    return 0


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
