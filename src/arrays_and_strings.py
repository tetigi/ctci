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
