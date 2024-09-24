# boilerplate_start begin

import sys


def __compiler__with_regular_args(func):
    def wrapper(*args):
        n_args = __stack.pop()
        args = [__stack.pop() for i in range(n_args)]
        func(*args)

    return wrapper


@__compiler__with_regular_args
def __builtin__printf(format, *args):
    sys.stdout.write(format % args)
    __intrinsic__stack_push(0)


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

# Builtin function: printf
def __userf__6():
	__builtin__printf()
# User function: add_two_numbers
def __userf__7():
	__intrinsic__stack_pop()
	_2_a = __intrinsic__stack_pop()
	_3_b = __intrinsic__stack_pop()
	__intrinsic__stack_push(_3_b)
	__intrinsic__stack_push(_2_a)
	__intrinsic__stack_add()
	_4_c = __intrinsic__stack_pop()
	__intrinsic__stack_push(_4_c)
	_5_k = __intrinsic__stack_pop()
	__intrinsic__stack_push(_4_c)
	__intrinsic__stack_push(_3_b)
	__intrinsic__stack_push(_2_a)
	__intrinsic__stack_push(__global_data[0])
	__intrinsic__stack_push(4)
	# Procedure call: printf
	__userf__6()
	__intrinsic__stack_pop()

	__intrinsic__stack_push(0)
	pass


def __setup():
	global __global_data
	__global_data = list(range(1))
	__global_data[0] = "%i + %i = %i\n"

def __main():
	__intrinsic__stack_push(1)
	_0_a = __intrinsic__stack_pop()
	__intrinsic__stack_push(_0_a)
	__intrinsic__stack_push(5)
	__intrinsic__stack_add()
	_1_b = __intrinsic__stack_pop()
	__intrinsic__stack_push(_1_b)
	__intrinsic__stack_push(_0_a)
	__intrinsic__stack_push(2)
	# Procedure call: add_two_numbers
	__userf__7()
	__intrinsic__stack_pop()

	pass

# boilerplate_exit begin


def __push_init_args():
    import sys

    args = [
        sys.argv,
        len(sys.argv),
    ]

    for arg in args:
        __intrinsic__stack_push(arg)

    __intrinsic__stack_push(len(args))


if __name__ == "__main__":
    import sys

    __push_init_args()
    __setup()

    __push_init_args()
    status = __main()

    print("stack", __stack)

    sys.exit(status)


# boilerplate_exit end

