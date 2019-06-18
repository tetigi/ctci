def number_swapper(a, b):
    if b > a:
        a = b - a
        b = b - a
        a = a + b
    else:
        a = a - b
        b = a + b
        a = b - a

    return (a, b)

def word_freqs(words, target_word):
    count = 0
    for word in words:
        if word == target_word:
            count += 1
    return count

def intersection(start1, end1, start2, end2):
    m1 = (end1[0] - start1[0]) / (end1[1] - start1[1])
    m2 = (end2[0] - start2[0]) / (end2[1] - start2[1])

    c1 = start1[1] - m1 * start1[0]
    c2 = start2[1] - m2 * start2[0]

    if m1 == m2:
        return c1 == c2

    y = (m1 * c2 - c1 * m2) / (m1 - m2)
    x = (c2 - c1) / (m1 - m2)

    return (max(start1[1], end1[1]) >= y >= min(start1[1], end1[1])) \
            and (max(start2[1], end2[1]) >= y >= min(start2[1], end2[1])) \
            and (max(start1[0], end1[0]) >= x >= min(start1[0], end1[0])) \
            and (max(start2[0], end2[0]) >= x >= min(start2[0], end2[0]))

def tic_tac_win(board):
    for i in range(len(board)):
        row = set(board[i])
        if len(set(row)) == 1 and row.pop() != '':
            return True

        col = set(board[j][i] for j in range(len(board)))
        if len(col) == 1 and col.pop() != '':
            return True

    diag1 = set(board[i][j] for i in range(len(board)) for j in range(len(board)))
    diag2 = set(board[i][len(board) - j - 1] for i in range(len(board)) for j in range(len(board)))

    return (len(diag1) == 1 and diag1.pop() != '') or (len(diag2) == 1 and diag2.pop() != '')

def factorial_zeros(n):
    return int(n / 5)

def smallest_difference(xs, ys):
    xs = sorted(xs)
    ys = sorted(ys)

    x_i = 0
    y_i = 0

    min_diff = (xs[0], ys[0])

    def diff(pair):
        return abs(pair[0] - pair[1])

    while x_i < len(xs) and y_i < len(ys):
        if xs[x_i] < ys[y_i]:
            x_i += 1
        elif ys[y_i] < xs[y_i]:
            y_i += 1
        else:
            return (xs[x_i], ys[y_i])

        if diff((xs[x_i], ys[y_i])) < diff(min_diff):
            min_diff = (xs[x_i], ys[y_i])

    return min_diff

def number_max(a, b):
    return ((int(a / b) * a) / int(a / b)) + ((int(b / a) * b) / int(b / a))

class Node:
    def __init__(self, value):
        self.value = value
        self.next = None
        self.prev = None

class LRUCache:
    def __init__(self, max_size):
        self.max_size = max_size
        self.data = {}
        self.node_map = {}
        self.first = None
        self.last = None

    def get(self, k):
        if k in self.data:
            node = self.node_map[k]
            if node.prev:
                node.prev.next = node.next

            if node.next:
                node.next.prev = node.prev

            self.first.prev = node
            self.first = node
            return self.data[k]

    def cache(self, k, v):
        if k in self.data or len(self.data) < self.max_size:
            self.data[k] = v

            node = Node(k)
            self.node_map[k] = node
            if self.last:
                self.last.next = node
                node.prev = self.last
                self.last = node
            else:
                self.first = node
                self.last = node
        else:
            self.data.pop(self.last.value)
            self.node_map.pop(self.last.value)
            if len(self.data) == 1:
                self.first = None
                self.last = None
            else:
                last_but_one = self.last.prev
                self.last.prev = None
                last_but_one.next = None
                self.last = last_but_one
            self.data[k] = v

