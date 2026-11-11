# Point density audit

Calculated 11 November 2026 from all eight `assets/images/*-points.svg` files and the current layout in `styles/global.css`. The initial audit is preserved below; the implemented reduction is recorded first.

## Implemented reduction

Plants and flag now use one point per occupied coarse cell, with opaque-pixel centroid placement and averaged color. Thin occupied cells are retained, so counts exceed a simple area-scaling estimate. Actual neighbor distances vary; the 2.225px pitch below is the nominal cell width, matching the current 310px desktop cat. Rectangular density remains higher for plants because their silhouettes fill more of their viewBoxes.

| Artwork | Original points | New points | New points / 1,000 px² | Nominal pitch (CSS px) |
| --- | ---: | ---: | ---: | ---: |
| Plant 1 | 12,164 | 582 | 149.6 | 2.225 |
| Plant 2 | 10,944 | 504 | 151.2 | 2.225 |
| Plant 3 | 5,892 | 305 | 170.0 | 2.225 |
| Plant 4 | 11,473 | 540 | 138.6 | 2.225 |
| Plant 5 | 9,324 | 453 | 169.4 | 2.225 |
| Colombia flag | 6,043 | 363 | 76.7 | 2.225 |
| Sleeping cat (unchanged) | 6,305 | 6,305 | 65.6 | 2.225 |
| Sleep marks (unchanged) | 39 | 39 | 30.5 | 1.800 |

Plants plus flag: **55,840 → 2,747 points (95.1% fewer)**. All artwork combined: **62,184 → 9,091 points (85.4% fewer)**. Plant and flag dot diameters are now 3.164 CSS pixels, matching the baseline cat. Dimensions and CSS boxes remain unchanged. The original figures and planning estimates below describe the pre-reduction assets.

The same responsive scaling applies after reduction: at a 1,024px viewport plants have about 2.057px nominal pitch. At ≥1,500px the cat grows to 2.512px pitch while plants/flag stay at 2.225px. Plants and flag remain hidden at ≤720px.

Regenerate with `swift scripts/generate-decoration-points.swift` from the repository root. It uses AppKit already used by the existing authoring tools, adds no dependencies, and does no work at website runtime. The sampler scans source pixels and connected components linearly; sorting output cells adds a small O(k log k) step for k occupied cells.

## How to compare

- **Point count:** number of SVG circles. The script also checks counts, positions, radii, and dimensions against the matching animation JSON.
- **Displayed image size:** the SVG viewBox scaled to fit its CSS box with `object-fit: contain`. Excludes letterboxing, but includes transparent space inside the viewBox. Rotation does not change area.
- **Density:** `point count / (displayed image width × displayed image height) × 1,000`, in points per 1,000 square CSS pixels. Larger means more points packed into the displayed image rectangle.
- **Grid pitch (nominal for centroid sampling):** `source grid spacing × display scale`, in CSS pixels. Smaller means more closely spaced dots. This is the sampling grid step, not the measured distance between every pair of neighboring dots; empty source pixels leave holes.
- **Dot diameter:** `2 × source radius × display scale`. Distinguishes the dot size from its spacing.

Density depends on how much of the image is occupied by the subject. Use grid pitch alongside it when comparing different silhouettes. These values describe resting artwork, not animated displacement, frame rate, or a count of non-overlapping painted pixels. Overlapping circles can produce densities above one point per square pixel.

## Original desktop baseline: viewport width 1,440 CSS pixels

| Artwork | Points | Displayed image (CSS px) | Points / 1,000 px² | Grid pitch (px) | Dot diameter (px) |
| --- | ---: | ---: | ---: | ---: | ---: |
| Colombia flag | 6,043 | 70.0 × 67.6 | 1,276.1 | 0.492 | 0.700 |
| Plant 1 | 12,164 | 63.0 × 61.7 | 3,127.1 | 0.452 | 0.643 |
| Plant 2 | 10,944 | 52.9 × 63.0 | 3,283.7 | 0.452 | 0.643 |
| Plant 3 | 5,892 | 28.5 × 63.0 | 3,283.2 | 0.452 | 0.643 |
| Plant 4 | 11,473 | 61.8 × 63.0 | 2,944.7 | 0.452 | 0.643 |
| Plant 5 | 9,324 | 42.5 × 63.0 | 3,486.3 | 0.452 | 0.643 |
| Sleep marks | 39 | 80.0 × 16.0 | 30.5 | 1.800 | 1.280 |
| Sleeping cat | 6,305 | 310.0 × 310.0 | 65.6 | 2.225 | 3.164 |

Total: **62,184 points**. The five plants account for **49,797 (80.1%)**. Plants plus flag account for **55,840 (89.8%)**.

The plants' grid is about **4.9 times tighter per axis** than the cat's, or **24.2 times as many grid locations per unit area**. The flag's grid is about **4.5 times tighter per axis**, or **20.4 times as many grid locations per unit area**. This grid comparison isolates sampling scale from silhouette coverage. Raw rectangular densities are about 45–53 times the cat's for plants, partly because plant viewBoxes contain less empty space.

## Original responsive sizing

Values above are calculated from CSS, not measured in a browser. The script mirrors the current site width, padding, rail, gaps, breakpoints, and artwork sizing. Keep its `boxes()` function aligned with future CSS changes.

- At a 1,024px viewport, plants shrink to 58.24px boxes and their grid pitch drops to about 0.418px. Smaller desktop layouts can pack them still more tightly.
- At widths of 721–1,000px, another layout rule changes the available panel width; sizes need recalculating for the specific viewport.
- At widths up to 720px, plants, flag, and sleep marks are hidden. Their displayed density is undefined; their source data still exists.
- At a 390px viewport, the cat box is 179.4px square: about 195.9 points per 1,000px², 1.288px pitch, and 1.831px dot diameter.
- At widths of at least 1,500px, the cat becomes 350px square with about 2.512px pitch. Plants and flag remain capped at their desktop sizes.

All measurements use CSS pixels. Device pixel ratio changes physical screen pixels, not these layout densities. No browser rendering or animation performance benchmark was run for this audit.

## Original planning estimate

Choose a desired **displayed grid pitch**, then derive source sampling separately for each illustration:

`new source grid spacing = desired CSS-pixel pitch / display scale`

For example, matching the current desktop cat's 2.225px pitch would require roughly 44.6-unit grids for the plants and 40.7 units for the flag, rather than 9 units. Holding silhouette coverage approximately constant, point counts scale with `(old pitch / new pitch)²`: the five plants together would fall from about 49,800 to about 2,050 points, and the flag from about 6,000 to about 300. These are planning estimates, not regenerated or visually approved assets. Fine stems, edges, and color details need visual review; dot radii must also be chosen to retain the desired coverage.

When changing assets, regenerate SVG and matching JSON together so the static and animated versions agree. A single desktop pitch will become smaller on mobile; decide the desired responsive appearance before selecting final sampling.

## Recalculate

From the repository root:

```sh
python3 scripts/analyze-point-density.py
python3 scripts/analyze-point-density.py --viewport 390
python3 scripts/analyze-point-density.py --viewport 1024
python3 scripts/analyze-point-density.py --csv docs/point-density.csv
```

The checked-in CSV contains the current 1,440px measurements. No extra packages are required. SVG/JSON structural consistency checks run on every invocation; calculations stop if those checks fail.
