#!/usr/bin/env python3
"""
Generate Rust constant declarations for PNG flag files in a directory.

Usage: python3 gen_flag_consts.py <directory>
"""

import os
import sys


def main():
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} <directory>", file=sys.stderr)
        sys.exit(1)

    directory = sys.argv[1]

    if not os.path.isdir(directory):
        print(f"Error: '{directory}' is not a directory.", file=sys.stderr)
        sys.exit(1)

    png_files = sorted(
        f for f in os.listdir(directory)
        if f.lower().endswith(".png")
    )

    if not png_files:
        print(f"No PNG files found in '{directory}'.", file=sys.stderr)
        sys.exit(1)

    for filename in png_files:
        base = os.path.splitext(filename)[0]
        const_name = base.upper()
        print(
            f'pub const FLAG_{const_name}: &[u8] = '
            f'include_bytes!("../../images/countryflags/{base}.png").as_slice();'
        )


if __name__ == "__main__":
    main()