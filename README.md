# USync App

A simple Ubuntu application that provides both:
- **CLI mode** for terminal usage
- **GUI mode** using Tkinter

## Run locally

```bash
python3 -m usync_app.main --name Alice
python3 -m usync_app.main add 4 5
python3 -m usync_app.main gui
```

## Install as a Python package

```bash
python3 -m pip install .
usync-app --name Alice
usync-app add 4 5
usync-app gui
```

## Build a `.deb` package

```bash
sudo apt-get install -y build-essential devscripts debhelper dh-python python3-all python3-setuptools pybuild-plugin-pyproject

dpkg-buildpackage -us -uc -b
```

The generated `.deb` will appear in the parent directory, e.g. `../usync-app_0.1.0-1_all.deb`.
