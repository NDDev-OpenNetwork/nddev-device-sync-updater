#!/usr/bin/env python3
"""Exercise the installed native owner on a private no-mutation policy fixture."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

if sys.platform != "linux":
    raise SystemExit("This isolated provider qualification currently requires Linux")
ROOT = Path(__file__).resolve().parent.parent
OWNER = 'updater'
binary = Path(os.environ["NDS_TEST_" + OWNER.upper()]).resolve(strict=True)
build = subprocess.run(["cargo", "test", "--locked", "--test", "provider", "--no-run", "--message-format=json"], cwd=ROOT, text=True, capture_output=True, check=True, timeout=180)
executables = [item["executable"] for line in build.stdout.splitlines() if (item := json.loads(line)).get("reason") == "compiler-artifact" and item.get("executable")]
assert len(executables) == 1
with tempfile.TemporaryDirectory(prefix="nds-" + OWNER + "-") as temporary:
    home = Path(temporary)
    env = os.environ.copy()
    env["HOME"] = str(home)
    env["NDS_ISOLATED_PROVIDER"] = "1"
    for key, directory in [("XDG_CONFIG_HOME", "config"), ("XDG_STATE_HOME", "state"), ("XDG_DATA_HOME", "data"), ("XDG_CACHE_HOME", "cache")]:
        (home / directory).mkdir(mode=0o700)
        env[key] = str(home / directory)
    for key in list(env):
        if key.startswith("RLDYOUR_"):
            del env[key]
    policy = home / "config" / ("rldyour-" + OWNER) / "config.toml"
    policy.parent.mkdir(mode=0o700)
    policy.write_text('schema = 2\nchannel = "signed-gds"\ncommand_timeout_seconds = 2\n[gds]\nenabled = false\napplications = false\n[apt]\nobserve = true\napply = false\n[native]\nhomebrew = false\nflatpak = false\n')
    # Only observations are enabled. The real native owner creates its own
    # report in this fixture; no invented run receipt or real user report.
    # A native observation can legitimately fail (for example its finite apt
    # deadline). Keep that actual report; status must preserve the failure too.
    report_path = home / "native-report.json"
    with report_path.open("xb") as output:
        result = subprocess.run([str(binary), "--config", str(policy), 'apply', "--json"],
                                env=env, stdout=output, stderr=subprocess.DEVNULL, timeout=30)
    if result.returncode not in (0, 1) or report_path.stat().st_size > 1_048_576:
        raise RuntimeError("unexpected native updater completion")
    report = json.loads(report_path.read_bytes())
    if not report.get("results") or result.returncode != int(any(not item["ok"] for item in report["results"])):
        raise RuntimeError("native updater exit/report mismatch")
    env["NDS_PROVIDER_REPORT"] = str(report_path)
    def state():
        return {str(path.relative_to(home)): hashlib.sha256(path.read_bytes()).hexdigest() for path in (home / "state").rglob("*") if path.is_file()}
    before = state()
    assert before, "native owner did not create its fixture report"
    subprocess.run([executables[0], "--ignored", "--nocapture"], env=env, check=True, timeout=120)
    assert before == state(), "read-only adapter changed native owner state"
print("Real isolated native provider acceptance passed; owner state unchanged")
