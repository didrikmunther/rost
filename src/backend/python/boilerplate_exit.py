# boilerplate_exit begin

if __name__ == "__main__":
    import sys

    argc = len(sys.argv)
    argv = sys.argv

    __setup()
    sys.exit(__main(argc, argv))


# boilerplate_exit end
