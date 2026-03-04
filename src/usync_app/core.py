from datetime import datetime


def get_status_message(name: str = "User") -> str:
    now = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    return f"Hello, {name}! USync is running at {now}."


def add_numbers(a: float, b: float) -> float:
    return a + b
