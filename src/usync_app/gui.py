import tkinter as tk
from tkinter import ttk

from .core import add_numbers, get_status_message


def run_gui() -> None:
    root = tk.Tk()
    root.title("USync App")
    root.geometry("420x240")

    frame = ttk.Frame(root, padding=16)
    frame.pack(fill="both", expand=True)

    status_var = tk.StringVar(value=get_status_message())

    ttk.Label(frame, text="USync GUI", font=("Ubuntu", 16, "bold")).pack(anchor="w")
    ttk.Label(frame, textvariable=status_var, wraplength=380).pack(anchor="w", pady=(8, 16))

    input_frame = ttk.Frame(frame)
    input_frame.pack(anchor="w", pady=(0, 8))

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

    root.mainloop()
