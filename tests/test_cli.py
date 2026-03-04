import io
import unittest
from contextlib import redirect_stdout

from usync_app.cli import run_cli


class TestCli(unittest.TestCase):
    def test_default_status_message(self) -> None:
        buffer = io.StringIO()
        with redirect_stdout(buffer):
            exit_code = run_cli([])

        self.assertEqual(exit_code, 0)
        self.assertIn("USync is running", buffer.getvalue())

    def test_add_subcommand(self) -> None:
        buffer = io.StringIO()
        with redirect_stdout(buffer):
            exit_code = run_cli(["add", "1.5", "2.5"])

        self.assertEqual(exit_code, 0)
        self.assertEqual(buffer.getvalue().strip(), "4.0")


if __name__ == "__main__":
    unittest.main()
