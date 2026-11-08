import pathlib
import sys

LOCALES = pathlib.Path(__file__).resolve().parent.parent / "locales"


def keys(path):
    return {line.split(":", 1)[0] for line in path.read_text().splitlines() if ":" in line}


reference = keys(LOCALES / "en.yml")
failed = False
for path in sorted(LOCALES.glob("*.yml")):
    found = keys(path)
    missing, extra = reference - found, found - reference
    if missing or extra:
        failed = True
        print(f"{path.name}: missing {sorted(missing)} extra {sorted(extra)}")
sys.exit(1 if failed else 0)
