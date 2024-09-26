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
def __userf__9():
	__builtin__printf()
# User function: add_two_things
def __userf__10():
	__intrinsic__stack_pop()
	_5_format = __intrinsic__stack_pop()
	_6_a = __intrinsic__stack_pop()
	_7_b = __intrinsic__stack_pop()
	__intrinsic__stack_push(_7_b)
	__intrinsic__stack_push(_6_a)
	__intrinsic__stack_add()
	_8_c = __intrinsic__stack_pop()
	__intrinsic__stack_push(_8_c)
	__intrinsic__stack_push(_7_b)
	__intrinsic__stack_push(_6_a)
	__intrinsic__stack_push(_5_format)
	__intrinsic__stack_push(4)
	# Procedure call: printf
	__userf__9()
	__intrinsic__stack_pop()

	__intrinsic__stack_push(0)
	pass

# Builtin function: printf
def __userf__15():
	__builtin__printf()
# User function: add_two_things
def __userf__16():
	__intrinsic__stack_pop()
	_11_format = __intrinsic__stack_pop()
	_12_a = __intrinsic__stack_pop()
	_13_b = __intrinsic__stack_pop()
	__intrinsic__stack_push(_13_b)
	__intrinsic__stack_push(_12_a)
	__intrinsic__stack_add()
	_14_c = __intrinsic__stack_pop()
	__intrinsic__stack_push(_14_c)
	__intrinsic__stack_push(_13_b)
	__intrinsic__stack_push(_12_a)
	__intrinsic__stack_push(_11_format)
	__intrinsic__stack_push(4)
	# Procedure call: printf
	__userf__15()
	__intrinsic__stack_pop()

	__intrinsic__stack_push(0)
	pass


def __setup():
	global __global_data
	__global_data = list(range(4))
	__global_data[0] = "abc"
	__global_data[1] = "%i + %i = %i\n"
	__global_data[2] = "def"
	__global_data[3] = "%s + %s = %s\n"

def __main():
	__intrinsic__stack_push(1)
	_2_a = __intrinsic__stack_pop()
	__intrinsic__stack_push(5)
	_3_c = __intrinsic__stack_pop()
	__intrinsic__stack_push(__global_data[0])
	_4_strvar = __intrinsic__stack_pop()
	__intrinsic__stack_push(_3_c)
	__intrinsic__stack_push(_2_a)
	__intrinsic__stack_push(__global_data[1])
	__intrinsic__stack_push(3)
	# Procedure call: add_two_things
	__userf__10()
	__intrinsic__stack_pop()
	__intrinsic__stack_push(__global_data[2])
	__intrinsic__stack_push(_4_strvar)
	__intrinsic__stack_push(__global_data[3])
	__intrinsic__stack_push(3)
	# Procedure call: add_two_things
	__userf__16()
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

