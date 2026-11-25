"""Render the pages beside this script into promo/out/, by where each image goes, with Microsoft Edge (which comes
with Windows) in headless mode.

    social.html    -> github/social-preview.png   1280 x 640, GitHub's social preview, uploaded by hand in the repo's
                                                  Settings -> Social preview whenever it changes
    portfolio.html -> portfolio/portfolio.png     1080 x 1080, a tile for the portfolio section of a job profile
                                                  (Boss 直聘, 猎聘, 脉脉), where it's mostly seen as a small square thumbnail

Both are uploaded by hand, so git keeps them.

Run from the repo root: python promo/generator/render.py
"""

from __future__ import annotations

import os
import struct
import subprocess
import sys
from pathlib import Path

GENERATOR = Path(__file__).resolve().parent
OUT = GENERATOR.parent / "out"
EDGE_PROFILE = Path(os.environ.get("TEMP", OUT)) / "docs-generator-edge"   # its own profile, so a running Edge isn't involved

# Each image: page, file in promo/out/, size in CSS pixels, and the most bytes it may have.
PAGES = [
    ("social.html", "social-preview.png", 1280, 640, 1_000_000),   # GitHub's limit is 1 MB
    ("portfolio.html", "portfolio.png", 1080, 1080, 2_000_000),
]


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


def render(edge: str, page: str, name: str, width: int, height: int, max_bytes: int) -> None:
    output = OUT / name
    output.parent.mkdir(parents=True, exist_ok=True)
    output.unlink(missing_ok=True)
    # --virtual-time-budget lets the page load its images and fonts before the shot is taken.
    subprocess.run(
        [
            edge, "--headless=new", "--disable-gpu", "--hide-scrollbars", "--allow-file-access-from-files",
            f"--user-data-dir={EDGE_PROFILE}", "--force-device-scale-factor=1",
            f"--window-size={width},{height}", "--virtual-time-budget=5000", f"--screenshot={output}",
            (GENERATOR / page).as_uri(),
        ],
        check=False,
        capture_output=True,
    )
    if not output.is_file():
        raise SystemExit(f"Edge didn't write promo/out/{name}.")
    size = png_size(output)
    if size != (width, height):
        raise SystemExit(f"promo/out/{name} is {size[0]} x {size[1]}, expected {width} x {height}.")
    if output.stat().st_size > max_bytes:
        raise SystemExit(f"promo/out/{name} is {output.stat().st_size:,} bytes, over {max_bytes:,}.")
    print(f"promo/out/{name}  {size[0]} x {size[1]}  {output.stat().st_size:,} bytes")


def main() -> None:
    edge = find_edge()
    for page in PAGES:
        render(edge, *page)


if __name__ == "__main__":
    if sys.platform != "win32":
        raise SystemExit("render.py runs on Windows, with Microsoft Edge.")
    main()
