def is_unique(s):
    return len(set(s)) == len(s)

def is_unique_no_structure(s):
    for (i, c1) in enumerate(s):
        for (_, c2) in enumerate(s[(i+1):]):
            if c1 == c2:
                return False

    return True

from collections import defaultdict

def check_permutations(s1, s2):
    counts1 = defaultdict(int);
    counts2 = defaultdict(int);

    for c in s1:
        counts1[c] += 1

    for c in s2:
        counts2[c] += 1

    return counts1 == counts2

from itertools import takewhile

def urlify(s):
    for i in range(len(s)-1, -1, -1):
        if s[i] != ' ':
            cursor = i
            break

    i = len(s) - 1
    while i > 0:
        if s[cursor] == ' ':
            s[i - 2: i + 1] = '%20'
            i -= 3
        else:
            s[i] = s[cursor]
            i -= 1
        cursor -= 1

def palindrome_perm(s):
    counts = defaultdict(int)
    s = list(filter(lambda x: x != ' ', s))

    for c in s:
        counts[c] += 1

    num_odd = len([v for v in counts.values() if v % 2 == 1])

    if len(s) % 2 == 1:
        return num_odd == 1
    else:
        return num_odd == 0
