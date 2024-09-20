# boilerplate_start begin

import sys


def __compiler__with_regular_args(func):
    def wrapper(*args):
        n_args = __stack.pop()
        args = reversed([__stack.pop() for i in range(n_args)])
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

def __user__printf():
	__intrinsic__stack_pop()
	_0_format = __intrinsic__stack_pop()

def __user__add_two_numbers():
	__intrinsic__stack_pop()
	_2_a = __intrinsic__stack_pop()
	_3_b = __intrinsic__stack_pop()
	__intrinsic__stack_push(_3_b)
	__intrinsic__stack_push(_2_a)
	__intrinsic__stack_add()
	_4_c = __intrinsic__stack_pop()
	__intrinsic__stack_push(__global_data[0])
	__intrinsic__stack_push(_2_a)
	__intrinsic__stack_push(_3_b)
	__intrinsic__stack_push(_4_c)
	__intrinsic__stack_push(4)
	__user__printf()
	__intrinsic__stack_pop()


def __setup():
	global __global_data
	__global_data = list(range(1))
	__global_data[0] = "%i + %i = %i\n"

def __main():
	__intrinsic__stack_push(1)
	_6_a = __intrinsic__stack_pop()
	__intrinsic__stack_push(2)
	_7_b = __intrinsic__stack_pop()
	__intrinsic__stack_push(_6_a)
	__intrinsic__stack_push(_7_b)
	__intrinsic__stack_push(2)
	__user__add_two_numbers()
	__intrinsic__stack_pop()

# boilerplate_exit begin

if __name__ == "__main__":
    import sys

    __intrinsic__stack_push(sys.argv)
    __intrinsic__stack_push(len(sys.argv))

    __setup()
    sys.exit(__main())


# boilerplate_exit end

