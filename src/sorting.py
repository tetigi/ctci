def sorted_merge(a, b):
    a_i = len(b)
    b_i = 0
    cursor = 0

    for i in range(len(a) - len(b)):
        a[-1 - i] = a[-1 - len(b) - i]

    while cursor < len(a):
        if a_i >= len(a):
            a[cursor] = b[b_i]
            b_i += 1
        elif b_i >= len(b):
            a[cursor] = a[a_i]
            a_i += 1
        else:
            this_a = a[a_i]
            this_b = b[b_i]

            if this_a <= this_b:
                a[cursor] = this_a
                a_i += 1
            else:
                a[cursor] = this_b
                b_i += 1
        cursor += 1
    return a

from collections import defaultdict, Counter

def group_anagrams(xs):
    anagrams = defaultdict(list)

    for x in xs:
        anagrams[tuple(sorted(Counter(x).items()))].append(x)

    for xs in anagrams.values():
        for x in xs:
            yield x

def search_rotated(arr, n):
    # find starting point
    p = int(len(arr) / 2)
    step = int(p/2)
    while arr[p] > arr[(p - 1) % len(arr)] and arr[(p + 1) % len(arr)] > arr[p]:
        if arr[p] > arr[-1]:
            p += step
        else:
            p -= step

        step = int(step / 2)
        if p == 1:
            break

    if not arr[(p + 1) % len(arr)] > arr[p]:
        p += 1

    # bin search
    q = p + int(len(arr) / 2)
    step = int(len(arr) / 4)

    while step >= 0:
        print("Looking at", q, "which is", arr[q % len(arr)], step)
        if arr[q % len(arr)] == n:
            return q % len(arr)
        elif arr[q % len(arr)] > n:
            q -= step
        else:
            q += step
        if step == 0:
            break
        step = int(step / 2)

    return None

class Listy:
    def __init__(self, xs):
        self.data = xs

    def elementAt(self, i):
        return self.data[i] if i < len(self.data) else -1

def no_size_sorted(listy, x):
    # establish size in log(n)
    size = 1
    while listy.elementAt(size) != -1 and listy.elementAt(size) < x:
        size *= 2

    # now we know that x is contained within size / 2 and size
    step = int(size / 4)
    while listy.elementAt(size) != x:
        elem = listy.elementAt(size)
        if elem == -1 or elem > x:
            size -= step
        else:
            size += step
        step = max(1, int(step / 2))

    return size
