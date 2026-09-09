"""Package a native release binary, usage docs and a working authored example."""
import hashlib
import platform
import shutil
import subprocess
import tarfile
import tempfile
import tomllib
import zipfile
from pathlib import Path

root = Path(__file__).resolve().parents[1]
version = tomllib.loads((root / "Cargo.toml").read_text())["workspace"]["package"]["version"]
system = {"Linux": "linux", "Darwin": "macos", "Windows": "windows"}[platform.system()]
arch = {"AMD64": "x86_64", "arm64": "aarch64"}.get(platform.machine(), platform.machine())
binary_name = "lumaui.exe" if system == "windows" else "lumaui"
binary = root / "target" / "release" / binary_name
assert subprocess.check_output([str(binary), "--version"], text=True).strip() == f"lumaui {version}"
name = f"lumaui-v{version}-{system}-{arch}"
dist = root / "dist"
dist.mkdir(exist_ok=True)
with tempfile.TemporaryDirectory(prefix="lumaui-package-") as temporary:
    stage = Path(temporary) / name
    stage.mkdir()
    shutil.copy2(binary, stage / binary_name)
    for file in ["README.md", "LICENSE", "CHANGELOG.md"]:
        shutil.copy2(root / file, stage / file)
    (stage / "docs").mkdir()
    for file in ["USAGE.md", "LANGUAGE_SPEC.md"]:
        shutil.copy2(root / "docs" / file, stage / "docs" / file)
    shutil.copytree(root / "examples" / "minimal", stage / "examples" / "minimal", ignore=shutil.ignore_patterns("generated"))
    example = stage / "examples" / "minimal"
    subprocess.run([str(stage / binary_name), "validate", str(example)], check=True)
    subprocess.run([str(stage / binary_name), "build", str(example)], check=True)
    # Ship authored sources; build output above is a packaging smoke check.
    shutil.rmtree(example / "generated")
    if system == "windows":
        archive = dist / f"{name}.zip"
        with zipfile.ZipFile(archive, "w", zipfile.ZIP_DEFLATED) as out:
            for path in sorted(stage.rglob("*")):
                if path.is_file():
                    out.write(path, path.relative_to(stage.parent))
    else:
        archive = dist / f"{name}.tar.gz"
        with tarfile.open(archive, "w:gz") as out:
            out.add(stage, arcname=name)
digest = hashlib.sha256(archive.read_bytes()).hexdigest()
(dist / f"{archive.name}.sha256").write_text(f"{digest}  {archive.name}\n")
print(archive)
