# boilerplate_start begin

import sys


def __builtin__printf(format, *args):
    sys.stdout.write(format % args)


__stack = []
__global_data = []


def __intrinsic__stack_add():
    a = __stack.pop()
    b = __stack.pop()
    __stack.append(a + b)


def __intrinsic__stack_mul():
    a = __stack.pop()
    b = __stack.pop()
    __stack.append(a * b)


def __intrinsic__stack_push(value):
    __stack.append(value)


def __intrinsic__stack_pop():
    return __stack.pop()


def __intrinsic__stack_callf(func, n_args):
    args = reversed([__stack.pop() for i in range(n_args)])
    __stack.append(func(*args))


# boilerplate_start end
