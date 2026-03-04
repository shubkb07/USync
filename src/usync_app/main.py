import sys

from .cli import run_cli
from .gui import run_gui


def main() -> int:
    if len(sys.argv) > 1 and sys.argv[1] == "gui":
        run_gui()
        return 0
    return run_cli(sys.argv[1:])


if __name__ == "__main__":
    raise SystemExit(main())
