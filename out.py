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

# Builtin function: printf
def __userf__5():
	__builtin__printf()
# User function: add_two_numbers
def __userf__6():
	__intrinsic__stack_pop()
	_2_a = __intrinsic__stack_pop()
	_3_b = __intrinsic__stack_pop()
	__intrinsic__stack_push(_3_b)
	__intrinsic__stack_push(_2_a)
	__intrinsic__stack_add()
	_4_c = __intrinsic__stack_pop()
	__intrinsic__stack_push(_4_c)
	__intrinsic__stack_push(_3_b)
	__intrinsic__stack_push(_2_a)
	__intrinsic__stack_push(__global_data[1])
	__intrinsic__stack_push(4)
	# Procedure call: printf
	__userf__5()
	__intrinsic__stack_pop()

	pass


def __setup():
	global __global_data
	__global_data = list(range(2))
	__global_data[0] = "hej"
	__global_data[1] = "%i + %i = %i\n"

def __main():
	__intrinsic__stack_push(1)
	_0_a = __intrinsic__stack_pop()
	__intrinsic__stack_push(__global_data[0])
	_1_b = __intrinsic__stack_pop()
	__intrinsic__stack_push(_1_b)
	__intrinsic__stack_push(_0_a)
	__intrinsic__stack_push(2)
	# Procedure call: add_two_numbers
	__userf__6()
	__intrinsic__stack_pop()

	pass

# boilerplate_exit begin

if __name__ == "__main__":
    import sys

    __intrinsic__stack_push(sys.argv)
    __intrinsic__stack_push(len(sys.argv))

    __setup()
    sys.exit(__main())


# boilerplate_exit end

