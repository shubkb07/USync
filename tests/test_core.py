import unittest

from usync_app.core import add_numbers, get_status_message


class TestCore(unittest.TestCase):
    def test_add_numbers(self) -> None:
        self.assertEqual(add_numbers(2, 3), 5)

    def test_status_message_contains_name(self) -> None:
        message = get_status_message("Alice")
        self.assertIn("Alice", message)
        self.assertIn("USync is running", message)


if __name__ == "__main__":
    unittest.main()
