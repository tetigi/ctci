def triple_step(n):
# can either go up 1, 2, or 3 at each step
    seen = {}
    def f(m):
        if m >= n:
            return 0
        elif m == n - 1:
            return 1
        elif m == n - 2:
            return 2
        elif m in seen:
            return seen[m]
        else:
            result = 3 + max(f(m+1) - 1, 0) + max(f(m+2) - 1, 0) + max(f(m+3) - 1, 0)
            seen[m] = result
            return result
    return f(1)

# let's do a DFS
def robot_in_a_grid(r, c, off_limits):
    seen = set()
    def search(path, x, y):
        if (x, y) in seen:
            return None
        elif x >= r or y >= c:
            return None
        elif (x, y) in off_limits:
            seen.add((x, y))
            return None
        elif x == (r - 1) and y == (c - 1):
            return path
        else:
            result = search(path + ['r'], x + 1, y) or search(path + ['d'], x, y + 1)

            if not result:
                seen.add((x, y))

            return result
    return search([], 0, 0)

def magic_index(xs):
    def f(xs, i, j):
        pivot = int((i + j) / 2)
        pivot_val = xs[pivot]
        if pivot_val == pivot:
            return pivot
        elif pivot == i or pivot == j:
            return None
        elif pivot_val > pivot:
            return f(xs, i, pivot)
        else:
            return f(xs, pivot + 1, j)

    return f(xs, 0, len(xs))

def powerset(xs):
    if xs:
        x = xs[0]
        rest = list(powerset(xs[1:]))
        return rest + list(map(lambda rs: [x] + rs, rest))
    else:
        return [[]]

def recursive_multiply(a, b):
    if a == 1:
        return b
    elif b == 1:
        return a
    else:
        return a + recursive_multiply(a, b - 1)

def towers_of_hanoi(n):
    stacks = [list(range(n, 0, -1)), [], []]

    def other_stack_index(i, j):
        return set([0,1,2]).difference(set([i, j])).pop()

    def f(orig_stack, from_disc, target_stack):
        if from_disc == len(stacks[orig_stack]) - 1:
            stacks[target_stack].append(stacks[orig_stack].pop())
        else:
            other_stack = other_stack_index(orig_stack, target_stack)

            to_move = len(stacks[orig_stack][from_disc+1:])
            f(orig_stack, from_disc+1, other_stack)
            print(stacks)
            stacks[target_stack].append(stacks[orig_stack].pop())
            print(stacks)
            f(other_stack, len(stacks[other_stack]) - to_move, target_stack)
            print(stacks)

    f(0, 0, 2)

def perms_without_dupes(chars):
    if len(chars) == 1:
        return [chars]
    else:
        perms = perms_without_dupes(chars[1:])
        new_perms = []
        for perm in perms:
            for i in range(len(perm) + 1):
                new_perms.append(perm[0:i] + chars[0] + perm[i:])
        return new_perms

def parens(n):
    seen = set()

    def f(m):
        if m == 1:
            return ["()"]
        else:
            bs = f(m-1)
            result = []
            for b in bs:
                pre = f"(){b}"
                wrap = f"({b})"
                post = f"{b}()"
                if pre not in seen:
                    seen.add(pre)
                    result.append(pre)
                if post not in seen:
                    seen.add(post)
                    result.append(post)
                result.append(wrap)

            return result

    return f(n)

def paint_fill(image, start, colour):
    (sx, sy) = start
    colour_to_change = image[sx][sy]

    def f(p):
        print(p)
        (x, y) = p
        image[x][y] = colour
        directions = [(a, b) for a in [1, 0, -1]
                             for b in [1, 0, -1]
                             if (0 <= x + a < len(image)
                                and 0 <= y + b < len(image[x + a])
                                and image[x + a][y + b] == colour_to_change)]

        for (a, b) in directions:
            f((x + a, y + b))

    f((0,0))

def coins(n):
# 25, 10, 5, 1
    memo = {}
    coins = [25, 10, 5]

    def f(m):
        if m < 5:
            return 1
        elif m in memo:
            return memo[m]
        else:
            result = sum([f(m - coin) for coin in coins if m >= coin]) + 1
            memo[m] = result
            return result

    return f(n)

def eight_queens():
    failed = set()

    def clear_spots(spot, valid_spots):
        new_valid = valid_spots.copy()
        (x, y) = spot
        c1 = y - x
        c2 = y + x
        for i in range(8):
            new_valid.discard((x, i))
            new_valid.discard((i, y))
            if 0 <= i + c1 < 8:
                new_valid.discard((i, i + c1))
            if 0 <= c2 - i < 8:
                new_valid.discard((i, c2 - i))
        return new_valid

    def f(positions, valid_spots):
        if tuple(sorted(positions)) in failed:
            return None
        if len(positions) == 8:
            return [tuple(sorted(positions))]
        else:
            results = set()
            for spot in valid_spots:
                remaining = clear_spots(spot, valid_spots)
                result = f(positions + [spot], remaining)
                if result:
                    results.update(result)
            if results:
                return results
            else:
                failed.add(tuple(sorted(positions)))
                return None

    return f([], set((x, y) for x in range(8) for y in range(8)))

def eight_queens_2():
    results = []

    def place_queens(row, cols):
        if row == 8:
            results.append(cols.copy())
        else:
            for col in range(8):
                if check_valid(cols, row, col):
                    if len(cols) <= row:
                        cols.append(col)
                    else:
                        cols[row] = col
                    place_queens(row + 1, cols)

    def check_valid(cols, row1, col1):
        for row2 in range(row1):
            col2 = cols[row2]
            if col1 == col2:
                return False
            col_distance = abs(col1 - col2)
            row_distance = abs(row1 - row2)
            if col_distance == row_distance:
                return False
        return True

    place_queens(0, [])
    return results
