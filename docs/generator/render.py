"""Render GitHub's social preview, docs/social-preview.png, from social.html beside this script.

The card is shown when the repo's link is shared, and heads the README. GitHub takes it from the repo's
Settings -> Social preview, so upload it there again whenever it changes. Uses Microsoft Edge, which comes with
Windows, in headless mode.

Run from the repo root: python docs/generator/render.py
"""

from __future__ import annotations

import os
import struct
import subprocess
import sys
from pathlib import Path

GENERATOR = Path(__file__).resolve().parent
DOCS = GENERATOR.parent
OUTPUT = DOCS / "social-preview.png"
EDGE_PROFILE = Path(os.environ.get("TEMP", DOCS)) / "social-preview-edge"   # its own profile, so a running Edge isn't involved

# GitHub's social preview: 1280 x 640, under 1 MB.
WIDTH, HEIGHT = 1280, 640
MAX_BYTES = 1_000_000


def find_edge() -> str:
    for root in (os.environ.get("ProgramFiles(x86)"), os.environ.get("ProgramFiles")):
        if root:
            path = Path(root) / "Microsoft" / "Edge" / "Application" / "msedge.exe"
            if path.is_file():
                return str(path)
    raise SystemExit("Microsoft Edge wasn't found.")


def png_size(path: Path) -> tuple[int, int]:
    with path.open("rb") as f:
        header = f.read(24)
    return struct.unpack(">II", header[16:24])


def main() -> None:
    OUTPUT.unlink(missing_ok=True)
    # --virtual-time-budget lets the page load its images and fonts before the shot is taken.
    subprocess.run(
        [
            find_edge(), "--headless=new", "--disable-gpu", "--hide-scrollbars", "--allow-file-access-from-files",
            f"--user-data-dir={EDGE_PROFILE}", "--force-device-scale-factor=1",
            f"--window-size={WIDTH},{HEIGHT}", "--virtual-time-budget=5000", f"--screenshot={OUTPUT}",
            (GENERATOR / "social.html").as_uri(),
        ],
        check=False,
        capture_output=True,
    )
    if not OUTPUT.is_file():
        raise SystemExit("Edge didn't write docs/social-preview.png.")
    size = png_size(OUTPUT)
    if size != (WIDTH, HEIGHT):
        raise SystemExit(f"docs/social-preview.png is {size[0]} x {size[1]}, expected {WIDTH} x {HEIGHT}.")
    if OUTPUT.stat().st_size > MAX_BYTES:
        raise SystemExit(f"docs/social-preview.png is {OUTPUT.stat().st_size:,} bytes, over GitHub's 1 MB.")
    print(f"docs/social-preview.png  {size[0]} x {size[1]}  {OUTPUT.stat().st_size:,} bytes")


if __name__ == "__main__":
    if sys.platform != "win32":
        raise SystemExit("render.py runs on Windows, with Microsoft Edge.")
    main()
