# USync App

A simple Ubuntu application that provides both:
- **CLI mode** for terminal usage
- **GUI mode** using Tkinter
- **Clipboard history panel** at the top of the GUI

## Run locally

```bash
python3 -m venv .venv
source .venv/bin/activate
pip install -e .

usync-app --name Alice
usync-app add 4 5
usync-app gui
```

## GUI features

- Clipboard is monitored and recent copied text appears at the top list.
- Use **Capture Clipboard** to pull current clipboard text manually.
- Use **Copy Selected** to copy any history entry back to clipboard.
- Includes a quick add calculator in the same window.

## Build a `.deb` package

```bash
sudo apt-get install -y build-essential devscripts debhelper dh-python python3-all python3-setuptools pybuild-plugin-pyproject

dpkg-buildpackage -us -uc -b
```

The generated `.deb` will appear in the parent directory, e.g. `../usync-app_0.1.0-1_all.deb`.

## Desktop app launcher

After installing the `.deb`, Ubuntu app menu shows **USync Clipboard** launcher with icon.
You can open it directly from app search/menu.
