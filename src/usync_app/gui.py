import tkinter as tk
from tkinter import ttk

from .core import ClipboardHistory, add_numbers, get_status_message


def run_gui() -> None:
    root = tk.Tk()
    root.title("USync Clipboard")
    root.geometry("620x420")

    frame = ttk.Frame(root, padding=16)
    frame.pack(fill="both", expand=True)

    status_var = tk.StringVar(value=get_status_message())

    ttk.Label(frame, text="USync GUI", font=("Ubuntu", 16, "bold")).pack(anchor="w")
    ttk.Label(frame, textvariable=status_var, wraplength=580).pack(anchor="w", pady=(8, 12))

    # Clipboard area (top)
    clipboard_history = ClipboardHistory(max_items=20)
    clipboard_status = tk.StringVar(value="Clipboard monitor ready")

    ttk.Label(frame, text="Clipboard History", font=("Ubuntu", 12, "bold")).pack(anchor="w")

    history_list = tk.Listbox(frame, height=8)
    history_list.pack(fill="x", pady=(4, 8))

    btn_row = ttk.Frame(frame)
    btn_row.pack(fill="x", pady=(0, 6))

    def copy_selected() -> None:
        selected = history_list.curselection()
        if not selected:
            clipboard_status.set("No clipboard item selected")
            return

        value = history_list.get(selected[0])
        root.clipboard_clear()
        root.clipboard_append(value)
        clipboard_status.set("Selected item copied to clipboard")

    def refresh_history_view() -> None:
        history_list.delete(0, tk.END)
        for item in clipboard_history.items():
            history_list.insert(tk.END, item)

    def copy_current_clipboard() -> None:
        try:
            value = root.clipboard_get()
        except tk.TclError:
            clipboard_status.set("Clipboard does not contain text")
            return

        clipboard_history.add(value)
        refresh_history_view()
        clipboard_status.set("Captured current clipboard text")

    ttk.Button(btn_row, text="Capture Clipboard", command=copy_current_clipboard).pack(side="left")
    ttk.Button(btn_row, text="Copy Selected", command=copy_selected).pack(side="left", padx=(8, 0))

    ttk.Label(frame, textvariable=clipboard_status).pack(anchor="w", pady=(0, 12))

    # Calculator area
    ttk.Separator(frame, orient="horizontal").pack(fill="x", pady=(4, 10))
    ttk.Label(frame, text="Quick Add", font=("Ubuntu", 12, "bold")).pack(anchor="w")

    input_frame = ttk.Frame(frame)
    input_frame.pack(anchor="w", pady=(6, 8))

    a_var = tk.StringVar(value="1")
    b_var = tk.StringVar(value="2")
    result_var = tk.StringVar(value="Result: 3")

    ttk.Label(input_frame, text="A:").grid(row=0, column=0, padx=(0, 4))
    ttk.Entry(input_frame, textvariable=a_var, width=10).grid(row=0, column=1, padx=(0, 12))
    ttk.Label(input_frame, text="B:").grid(row=0, column=2, padx=(0, 4))
    ttk.Entry(input_frame, textvariable=b_var, width=10).grid(row=0, column=3)

    def calculate() -> None:
        try:
            total = add_numbers(float(a_var.get()), float(b_var.get()))
            result_var.set(f"Result: {total}")
        except ValueError:
            result_var.set("Result: invalid input")

    ttk.Button(frame, text="Calculate", command=calculate).pack(anchor="w")
    ttk.Label(frame, textvariable=result_var).pack(anchor="w", pady=(8, 0))

    # Poll clipboard so latest copy appears at top automatically.
    last_clipboard_text = {"value": ""}

    def poll_clipboard() -> None:
        try:
            current = root.clipboard_get()
        except tk.TclError:
            current = ""

        if current and current != last_clipboard_text["value"]:
            last_clipboard_text["value"] = current
            clipboard_history.add(current)
            refresh_history_view()
            clipboard_status.set("Clipboard updated")

        root.after(1200, poll_clipboard)

    poll_clipboard()
    root.mainloop()
