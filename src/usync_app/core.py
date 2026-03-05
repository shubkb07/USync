from __future__ import annotations

from datetime import datetime


def get_status_message(name: str = "User") -> str:
    now = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    return f"Hello, {name}! USync is running at {now}."


def add_numbers(a: float, b: float) -> float:
    return a + b


class ClipboardHistory:
    """In-memory clipboard history with deduplication."""

    def __init__(self, max_items: int = 20) -> None:
        self.max_items = max_items
        self._items: list[str] = []

    def add(self, text: str) -> None:
        value = text.strip()
        if not value:
            return

        if value in self._items:
            self._items.remove(value)

        self._items.insert(0, value)
        del self._items[self.max_items :]

    def items(self) -> list[str]:
        return list(self._items)
