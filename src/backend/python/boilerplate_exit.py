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
