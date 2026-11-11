#!/usr/bin/env python3
"""Audit point SVGs without dependencies or changing artwork.

Run from any directory. CSS sizing below mirrors styles/global.css; update it
when layout rules change. Sizes are calculated, not browser measurements.
"""

import argparse
import csv
import json
import math
from pathlib import Path
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[1]


def boxes(viewport):
    if viewport <= 720:
        panel = viewport - 48
    elif viewport <= 1000:
        panel = viewport - 60 - 170 - 40
    else:
        panel = min(viewport, 1300 if viewport >= 1500 else 1240) - 96 - 208
        panel -= 100 if viewport >= 1500 else 80
    cat = (min(180, max(140, viewport * .46)) if viewport <= 480
           else 280 if viewport <= 720 else 350 if viewport >= 1500 else 310)
    plant = (min(panel * .48, 63 * 5 + 16) - 16) / 5
    flag = min(70, (panel * .48 - 16) * .266667)
    return {"sleeping-cat": (cat, cat), "sleep-marks": (80, 16),
            "colombia-flag": (flag, flag * 1237 / 1280),
            **{f"plant-{i}": (plant, plant) for i in range(1, 6)}}


def artworks():
    for path in sorted((ROOT / "assets/images").glob("*-points.svg")):
        svg = ET.parse(path).getroot()
        x, y, width, height = map(float, svg.attrib["viewBox"].split())
        assert x == y == 0, f"Unexpected viewBox origin: {path.name}"
        circles = list(svg.iter("{http://www.w3.org/2000/svg}circle"))
        data = json.loads(path.with_suffix(".json").read_text())
        assert (width, height) == (data["width"], data["height"]), path.name
        assert len(circles) == len(data["points"]), path.name
        parents = {child: parent for parent in svg.iter() for child in parent}
        for circle, point in zip(circles, data["points"]):
            assert (float(circle.attrib["cx"]), float(circle.attrib["cy"])) == tuple(point[:2]), path.name
            assert float(circle.attrib["r"]) == data["radius"], path.name
            assert all(math.isfinite(value) for value in point), path.name
            element = circle
            while "fill" not in element.attrib and element in parents:
                element = parents[element]
            assert element.attrib["fill"].lower() == "#{:02x}{:02x}{:02x}".format(*map(int, point[2:])), path.name
            assert all(0 <= value <= 255 and value == int(value) for value in point[2:]), path.name
            radius = data["radius"]
            assert radius - 1e-8 <= point[0] <= width - radius + 1e-8, path.name
            assert radius - 1e-8 <= point[1] <= height - radius + 1e-8, path.name
        yield path.stem.removesuffix("-points"), width, height, len(circles), data


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--viewport", type=float, default=1440,
                        help="Viewport width in CSS pixels (default: 1440)")
    parser.add_argument("--csv", type=Path, help="Optional CSV output path")
    args = parser.parse_args()
    if args.viewport < 280:
        parser.error("Use a viewport of at least 280 CSS pixels")
    sizes = boxes(args.viewport)
    rows = []
    for name, width, height, count, data in artworks():
        bw, bh = sizes[name]
        scale = min(bw / width, bh / height)
        rw, rh = width * scale, height * scale
        visible = args.viewport > 720 or name == "sleeping-cat"
        rows.append(dict(artwork=name, visible=visible, points=count,
                         source_width=width, source_height=height,
                         box_width_css_px=round(bw, 3), box_height_css_px=round(bh, 3),
                         image_width_css_px=round(rw, 3), image_height_css_px=round(rh, 3),
                         points_per_1000_css_px2=round(count / (rw * rh) * 1000, 3) if visible else None,
                         grid_pitch_css_px=round(data["spacing"] * scale, 3) if visible else None,
                         dot_diameter_css_px=round(2 * data["radius"] * scale, 3) if visible else None))
    print(f"Viewport: {args.viewport:g} CSS px; sizes calculated from current CSS.")
    print("Hidden artwork has no displayed density. Image sizes exclude contain letterboxing.\n")
    print("| Artwork | Points | Image size (CSS px) | Points / 1,000 px² | Grid pitch (px) | Dot diameter (px) |")
    print("| --- | ---: | ---: | ---: | ---: | ---: |")
    for row in rows:
        if not row["visible"]:
            print(f"| {row['artwork']} | {row['points']:,} | hidden | — | — | — |")
            continue
        print(f"| {row['artwork']} | {row['points']:,} | {row['image_width_css_px']:.1f} × {row['image_height_css_px']:.1f} | {row['points_per_1000_css_px2']:.1f} | {row['grid_pitch_css_px']:.3f} | {row['dot_diameter_css_px']:.3f} |")
    print(f"\nTotal stored points: {sum(r['points'] for r in rows):,}; visible: {sum(r['points'] for r in rows if r['visible']):,}.")
    if args.csv:
        with args.csv.open("w", newline="") as output:
            writer = csv.DictWriter(output, fieldnames=list(rows[0]))
            writer.writeheader()
            writer.writerows(rows)


if __name__ == "__main__":
    main()
