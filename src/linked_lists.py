class Node:
    def __init__(self, value):
        self.value = value

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
