# colour-exercise-rs

This repository is mainly here as a personal exercise in exploring colour - including calculating *distance* and conversion between *colour spaces*.

The raw logic is under [`comparisons.rs`](./src/comparisons.rs) and [`conversions.rs`](./src/conversions.rs) if you'd like to check it out. Everything under
[`pixel/`](./src/pixel/) is mostly wrappers to make it easier to use colours.

Was done with the help of the `color.js` codebase as a reference point.

Currently, the following *conversions* are supported:
- RGB to/from HSL
- RGB to/from XYZ_65
- XYZ_D65 to/from XYZ_50
- XYZ_D50 to/from LAB
- LAB to/from LCH
- XYZ_D65 to/from OKLAB
- OKLAB to/from OKLCH

...and the following *distance algorithms*:
- RGB weighted euclidean
- CIE76
- CIE94
- CIEDE2000

Some benchmarks are included as well.