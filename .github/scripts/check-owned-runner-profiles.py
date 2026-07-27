#!/usr/bin/env python3
"""Reject hosted, dynamic, or noncanonical CI runner selectors."""

from __future__ import annotations

import re
import sys
from pathlib import Path


RUNS_ON = re.compile(r"^\s*runs-on\s*:\s*(?P<value>[^#]+?)(?:\s+#.*)?$")
LINUX = re.compile(r"^sylphx-linux-(?:control|standard|large|xlarge|2xlarge)$")
MACOS_SIZES = {"nano", "small", "standard", "large", "xlarge", "2xlarge"}


def unquote(value: str) -> str:
    return value.strip().strip("\"'")


def is_owned(value: str) -> bool:
    value = unquote(value)
    if LINUX.fullmatch(value):
        return True
    if not (value.startswith("[") and value.endswith("]")):
        return False
    labels = [unquote(label).lower() for label in value[1:-1].split(",") if label.strip()]
    return (
        len(labels) == 2
        and labels[0] == "self-hosted"
        and LINUX.fullmatch(labels[1]) is not None
    ) or (
        len(labels) == 4
        and labels[:3] == ["self-hosted", "sylphx", "macos"]
        and labels[3] in MACOS_SIZES
    )


def main() -> int:
    root = Path(__file__).resolve().parents[2]
    workflows = sorted((root / ".github" / "workflows").glob("*.y*ml"))
    errors: list[str] = []
    for workflow in workflows:
        for line, raw in enumerate(workflow.read_text(encoding="utf-8").splitlines(), 1):
            match = RUNS_ON.match(raw)
            if not match:
                continue
            value = match.group("value").strip()
            if "${{" in value or not is_owned(value):
                errors.append(f"{workflow.relative_to(root)}:{line}: forbidden runner selection {value!r}")
    if errors:
        print("\n".join(errors), file=sys.stderr)
        return 1
    print(f"OK: {len(workflows)} workflow(s) use static owned runner profiles")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
