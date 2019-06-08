class Node:
    def __init__(self, value):
        self.value = value
        self.next = None

class MinStack:
    def __init__(self):
        self._min = []
        self._stack = []

    def min(self):
        return self._min[0] if len(self._min) > 0 else None

    def push(self, val):
        if not self.min() or val <= self.min():
            self._min.append(val)
        self._stack.append(val)

    def pop(self):
        val = self._stack.pop()
        if val == self.min():
            self._min.pop()
        return val
