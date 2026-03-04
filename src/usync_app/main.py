import sys

from .cli import run_cli


def main() -> int:
    args = sys.argv[1:]
    if args and args[0] == "gui":
        from .gui import run_gui

        run_gui()
        return 0
    return run_cli(args)


if __name__ == "__main__":
    raise SystemExit(main())
