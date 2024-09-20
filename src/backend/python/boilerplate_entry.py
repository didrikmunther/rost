# boilerplate_start begin

import sys


def __compiler__with_regular_args(func):
    def wrapper(*args):
        n_args = __stack.pop()
        args = [__stack.pop() for i in range(n_args)]
        __stack.append(func(*args))

    return wrapper


@__compiler__with_regular_args
def __builtin__printf(format, *args):
    sys.stdout.write(format % args)


__stack = []
__global_data = []
__global_variables = {}


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


# boilerplate_start end
