def letters_and_numbers(xs):
    diffs = []
    current_diff = 0
    for x in xs:
        if x.isdigit():
            current_diff += 1
        else:
            current_diff -= 1
        diffs.append(current_diff)

    diff_map = {}
    for (i, diff) in enumerate(diffs):
        if diff not in diff_map:
            diff_map[diff] = (i, -1)
        else:
            (start, _) = diff_map[diff]
            diff_map[diff] = (start, i)

    return max(diff_map.values(), key=lambda x: x[1] - x[0])

def count_twos_dumb(n):
    return sum(str(i).count('2') for i in range(0, n+1))

def get_magnitude(n):
    count = 0
    while n > 0:
        count += 1
        n = int(n / 10)

    return count

def twos_magnitude(n):
    if n == 1:
        return 1
    else:
        twos = twos_magnitude(n - 1)
        return 9 * twos + twos + (10 ** (n-1))

def count_twos(n):
    magnitude = get_magnitude(n)
    return twos_magnitude(magnitude)

from collections import defaultdict

def baby_names(names, synonyms):
    syn_map = {}
    next_id = 0
    for (s1, s2) in synonyms:
        if s1 in syn_map:
            syn_map[s2] = s1
        elif s2 in syn_map:
            syn_map[s1] = s2
        else:
            syn_map[s1] = s2

    terminals = {}
    name_totals = defaultdict(int)
    for name in names:
        if name not in terminals:
            terminal = name
            encountered = []
            while terminal in syn_map:
                terminal = syn_map[terminal]
                encountered.append(terminal)
            for e in encountered:
                terminals[e] = terminal

        name_totals[terminal] += names[name]

    return name_totals

input_baby_names = {"john": 15, "jon": 12, "chris": 13, "kris": 4, "christopher": 19}
input_synonyms = [("jon", "john"), ("john", "johnny"), ("chris", "kris"), ("chris", "christopher")]
