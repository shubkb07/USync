import unittest

from usync_app.core import ClipboardHistory, add_numbers, get_status_message


class TestCore(unittest.TestCase):
    def test_add_numbers(self) -> None:
        self.assertEqual(add_numbers(2, 3), 5)

    def test_status_message_contains_name(self) -> None:
        message = get_status_message("Alice")
        self.assertIn("Alice", message)
        self.assertIn("USync is running", message)

    def test_clipboard_history_keeps_recent_unique_items(self) -> None:
        history = ClipboardHistory(max_items=3)
        history.add("one")
        history.add("two")
        history.add("one")
        history.add("three")
        history.add("four")
        self.assertEqual(history.items(), ["four", "three", "one"])


if __name__ == "__main__":
    unittest.main()
