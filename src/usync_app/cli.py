import argparse

from .core import add_numbers, get_status_message


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(prog="usync-app", description="USync Ubuntu GUI/CLI demo application")
    parser.add_argument("--name", default="User", help="Name to include in the status message")

    subparsers = parser.add_subparsers(dest="command")

    add_cmd = subparsers.add_parser("add", help="Add two numbers")
    add_cmd.add_argument("a", type=float)
    add_cmd.add_argument("b", type=float)

    return parser


def run_cli(args: list[str] | None = None) -> int:
    parser = build_parser()
    parsed = parser.parse_args(args=args)

    if parsed.command == "add":
        print(add_numbers(parsed.a, parsed.b))
    else:
        print(get_status_message(parsed.name))

    return 0
