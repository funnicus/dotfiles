"""Exercise real chezmoi diff/apply with an isolated home and fake Bitwarden."""

import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest


REPO = Path(__file__).resolve().parents[1]
TEMPLATES = {
    "private_dot_ssh/config.tmpl": ".ssh/config",
    "dot_config/private_ngrok/private_ngrok.yml.tmpl": ".config/ngrok/ngrok.yml",
    "Library/Application Support/private_ngrok/private_ngrok.yml.tmpl":
        "Library/Application Support/ngrok/ngrok.yml",
}


class BitwardenTemplatesTest(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.source = self.root / "source"
        self.destination = self.root / "home"
        self.destination.mkdir()
        for relative in [".chezmoiignore", *TEMPLATES]:
            target = self.source / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(REPO / relative, target)
        (self.source / "public.txt").write_text("public config\n")
        (self.root / "config.toml").write_text("")
        self.bin = self.root / "bin"
        self.bin.mkdir()
        bw = self.bin / "bw"
        bw.write_text("""#!/bin/sh
printf 'called\n' >> "$BW_TEST_CALLS"
if [ "$#" -ne 4 ] || [ "$1" != get ] || [ "$2" != notes ] || [ "$4" != --nointeraction ]; then
    echo 'Unexpected interactive Bitwarden lookup' >&2
    exit 2
fi
if [ "$BW_SESSION" != test-unlocked ]; then
    echo 'Vault is locked.' >&2
    exit 1
fi
case "$3" in
    ssh-test) printf 'Host private-test\n    HostName example.invalid\n' ;;
    ngrok-test) printf 'test-ngrok-token' ;;
    *) exit 3 ;;
esac
""")
        bw.chmod(0o755)
        self.calls = self.root / "calls"

    def chezmoi(self, command, platform, session=None):
        environment = dict(os.environ)
        environment.pop("BW_SESSION", None)
        if session is not None:
            environment["BW_SESSION"] = session
        environment["PATH"] = str(self.bin) + os.pathsep + environment["PATH"]
        environment["BW_TEST_CALLS"] = str(self.calls)
        return subprocess.run(
            ["chezmoi", "--config", str(self.root / "config.toml"),
             "--source", str(self.source), "--destination", str(self.destination),
             "--cache", str(self.root / "cache"),
             "--persistent-state", str(self.root / "state.boltdb"),
             "--override-data", json.dumps({
                 "chezmoi": {"os": platform},
                 "ssh": {"bitwardenItem": "ssh-test"},
                 "ngrok": {"bitwardenItem": "ngrok-test"},
             }), "--no-pager", "--no-tty", command],
            env=environment, stdin=subprocess.DEVNULL, capture_output=True,
            text=True, timeout=15,
        )

    def test_no_session_skips_secrets_and_preserves_existing_files(self):
        for relative in TEMPLATES.values():
            target = self.destination / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text("existing private config\n")
        for platform in ["darwin", "linux"]:
            for session in [None, ""]:
                with self.subTest(platform=platform, session=session):
                    for command in ["diff", "apply"]:
                        result = self.chezmoi(command, platform, session)
                        self.assertEqual(result.returncode, 0, result.stderr)
                    self.assertFalse(self.calls.exists())
                    for relative in TEMPLATES.values():
                        self.assertEqual((self.destination / relative).read_text(),
                                         "existing private config\n")
        self.assertEqual((self.destination / "public.txt").read_text(), "public config\n")

    def test_unlocked_session_renders_secrets(self):
        for platform in ["darwin", "linux"]:
            with self.subTest(platform=platform):
                result = self.chezmoi("diff", platform, "test-unlocked")
                self.assertEqual(result.returncode, 0, result.stderr)
                self.assertIn("Host private-test", result.stdout)
                self.assertEqual(result.stdout.count("authtoken: test-ngrok-token"),
                                 2 if platform == "darwin" else 1)

    def test_invalid_session_fails_without_prompting(self):
        for platform in ["darwin", "linux"]:
            with self.subTest(platform=platform):
                result = self.chezmoi("diff", platform, "test-invalid")
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("Vault is locked.", result.stderr)
                self.assertNotIn("interactive Bitwarden lookup", result.stderr)


if __name__ == "__main__":
    unittest.main()
