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

input_circus1 = [(65, 100), (70, 150), (56, 90), (75, 190), (60, 110), (68, 110)]
input_circus2 = [(65, 100), (70, 150), (56, 90), (75, 190), (60, 95), (68, 110)]

class Node:
    def __init__(self, value):
        self.children = set()
        self.w = value[0]
        self.h = value[1]
        self.visited = 0

    def __repr__(self):
        return f"({self.w}, {self.h})"

def get_max_tower(root, path):
    our_path = path + [root]
    max_path = our_path
    for child in root.children:
        new_path = get_max_tower(child, our_path)
        if len(new_path) > len(max_path):
            max_path = new_path
    return max_path

def circus_tower(ps):
    weight_ordering = sorted(ps)
    height_ordering = sorted(ps, key=lambda x: x[1])

    first = weight_ordering[0]
    rest = weight_ordering[1:]
    nodes = {}
    last = Node(first)
    nodes[first] = last
    to_visit = set(ps)

    while to_visit:
        n = to_visit.pop()
        if not nodes[n]:
            for (w, h) in rest:
                if w > last.w and h > last.h:
                    new = Node((w, h))
                    nodes[(w, h)] = new
                    last.children.add(new)
                    last = new

    first = height_ordering[0]
    rest = height_ordering[1:]
    last = Node(first) if first not in nodes else nodes[first]
    for (w, h) in rest:
        if w > last.w and h > last.h:
            if (w, h) in nodes:
                node = nodes[(w, h)]
                last.children.add(node)
                last = node
            else:
                new = Node((w, h))
                nodes[(w, h)] = new
                last.children.add(new)
                last = new

    height_first = height_ordering[0]
    weight_first = weight_ordering[0]

    max1 = get_max_tower(nodes[height_first], [])
    max2 = get_max_tower(nodes[weight_first], [])

    return max1 if len(max1) > len(max2) else max2

def circus_tower2(ps):
    ps = sorted(ps)

    max_seq = []
    best = []
    for i in range(len(ps)):
        best_i = best_from(ps[:i+1], best)
        best.append(best_i)
        if len(best_i) > len(max_seq):
            max_seq = best_i

    return max_seq

def best_from(ps, best_from):
    if len(ps) == 1:
        return ps

    longest = []
    p = ps[-1]
    for i in range(len(ps) - 1):
        best_i = best_from[i]
        (w, h) = best_i[-1]
        if w < p[0] and h < p[1]:
            maybe = best_i + [p]
            if len(maybe) > len(longest):
                longest = maybe

    return longest

from itertools import combinations_with_replacement

def kth_multiple(k):
    results = {3: 3, 5: 5, 7: 7}
    indices = {3: 0, 5: 0, 7: 0}
    nums = []
    seen = set()
    for j in range(k):
        (k, smallest) = min([(k, results[k]) for k in results], key=lambda x: x[1])
        nums.append(smallest)
        next_num = nums[indices[k]] * k
        while next_num in seen:
            indices[k] += 1
            next_num = nums[indices[k]] * k
        seen.add(next_num)
        results[k] = next_num

    return nums


