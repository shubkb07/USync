"""USync application package."""

__all__ = ["run_cli", "run_gui"]

from .cli import run_cli


def run_gui() -> None:
    """Launch the GUI lazily to avoid tkinter import in CLI contexts."""
    from .gui import run_gui as _run_gui

    _run_gui()
