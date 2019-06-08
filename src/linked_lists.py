class Node:
    def __init__(self, value):
        self.value = value

    def __len__(self):
        count = 0
        tmp = self
        while tmp != None:
            count += 1
            tmp = tmp.next
        return count

    def __repr__(self):
        if self.next:
            next = repr(self.next)
            return f"{self.value} -> {next}"
        else:
            return f"{self.value}"

from functools import reduce

def linked_list(xs):
    def acc(acc, x):
        n = Node(x)
        n.next = acc
        return n

    return reduce(acc, reversed(xs), None)

def remove_duplicates(xs):
    seen = set([xs.value])

    last = xs
    current = xs.next

    while current:
        if current.value in seen:
            last.next = current.next
            current = current.next
        else:
            seen.add(current.value)
            last = current
            current = current.next

def kth_to_last(k, xs):
    end_marker = xs

    for _ in range(k):
        end_marker = end_marker.next

    kth = xs

    while end_marker:
        end_marker = end_marker.next
        kth = kth.next

    return kth.value

def delete_in_middle(pointer):
    last = pointer
    next_node = pointer.next

    while next_node:
        last.value = next_node.value
        last = next_node
        next_node = next_node.next

    last.next = None

def partition(x, xs):
    left = xs
    right = xs.next

    while left.value < x:
        left = left.next
        right = right.next

    while right:
        if right.value < x:
            tmp = left.value
            left.value = right.value
            right.value = tmp
            left = left.next
        right = right.next

def sum_lists(n1, n2):
    carry = 0
    output = []

    while n1 or n2:
        n1_val = 0 if n1 == None else n1.value
        n2_val = 0 if n2 == None else n2.value
        res = n1_val + n2_val + carry
        carry = 0
        if res >= 10:
            carry = 1
            output.append(res % 10)
        else:
            output.append(res)
        n1 = n1.next if n1 != None else None
        n2 = n2.next if n2 != None else None

    if carry == 1:
        output.append(1)

    return linked_list(output)

def is_palindrome(xs):
    acc = xs
    tail_check = xs
    rev = None

    while tail_check:
        this_node = Node(acc.value)
        this_node.next = rev
        rev = this_node
        acc = acc.next
        tail_check = tail_check.next

        if tail_check:
            tail_check = tail_check.next
        else:
            # is odd, so trim rev
            rev = rev.next

    while acc:
        if acc.value != rev.value:
            return False
        else:
            acc = acc.next
            rev = rev.next

    return True

def get_intersection(xs, ys):
    len_xs = len(xs)
    len_ys = len(ys)
    if len_xs > len_ys:
        for _ in range(len_xs - len_ys):
            xs = xs.next

    while xs:
        if xs == ys:
            return xs
        xs = xs.next
        ys = ys.next
