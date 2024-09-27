# boilerplate_exit begin


def init_args():
    import sys

    args = [
        sys.argv,
        len(sys.argv),
    ]

    for arg in args:
        push(arg)

    push(len(args))


if __name__ == "__main__":
    import sys

    init_args()
    setup()

    init_args()
    status = __main()

    print("stack", stack)

    sys.exit(status)


# boilerplate_exit end
