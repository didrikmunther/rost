# boilerplate_exit begin

if __name__ == "__main__":
    import sys

    __intrinsic__stack_push(sys.argv)
    __intrinsic__stack_push(len(sys.argv))

    __setup()
    sys.exit(__main())


# boilerplate_exit end
