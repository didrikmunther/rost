# boilerplate_start begin

import sys


def stack_to_regular_args(func):
    def wrapper(*args):
        n_args = stack.pop()
        args = [stack.pop() for i in range(n_args)]
        func(*args)

    return wrapper


@stack_to_regular_args
def _builtin_printf(format, *args):
    sys.stdout.write(format % args)
    push(0)


stack = []
glob_data = []
glob_vars = {}


def add():
    a = stack.pop()
    b = stack.pop()
    stack.append(a + b)


def mul():
    a = stack.pop()
    b = stack.pop()
    stack.append(a * b)


def push(value):
    stack.append(value)


def pop():
    return stack.pop()


# boilerplate_start end
