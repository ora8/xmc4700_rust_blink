# Ersetzt den C-Linkschritt von PlatformIO durch "cargo build" und
# liefert das fertige Rust-ELF als firmware.elf an PlatformIO zurück.
# Danach laufen objcopy (firmware.bin/hex), Größenanzeige und Upload normal.
Import("env")
import os
import shutil
import subprocess

PROJ = env.subst("$PROJECT_DIR")
TARGET_TRIPLE = "thumbv7em-none-eabihf"   # Cortex-M4F (XMC4700)
CRATE_BIN = "xmc4700-blink"


def build_rust(target, source, env):
    cmd = ["cargo", "build", "--release"]
    print("Running:", " ".join(cmd))
    subprocess.check_call(cmd, cwd=PROJ)
    elf = os.path.join(PROJ, "target", TARGET_TRIPLE, "release", CRATE_BIN)
    shutil.copyfile(elf, str(target[0]))
    return None


env.Replace(LINKCOM=env.Action(build_rust, "Cargo build (Rust) -> $TARGET"))
