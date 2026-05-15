from __future__ import annotations

import html
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "screenshots"
OUT.mkdir(exist_ok=True)

EDGE_CANDIDATES = [
    Path(r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe"),
    Path(r"C:\Program Files\Microsoft\Edge\Application\msedge.exe"),
]


def edge_path() -> Path | None:
    for path in EDGE_CANDIDATES:
        if path.exists():
            return path
    return None


def run_cli(args: list[str]) -> tuple[str, int]:
    completed = subprocess.run(
        ["cargo", "run", "--quiet", "--", *args],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    output = completed.stdout.strip() or completed.stderr.strip()
    return output, completed.returncode


def write_scene(name: str, title: str, subtitle: str, command: str, output: str) -> Path:
    page = OUT / f"{name}.html"
    page.write_text(
        f"""<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>{html.escape(title)}</title>
    <style>
      body {{
        margin: 0;
        font-family: Inter, "Segoe UI", system-ui, sans-serif;
        background: linear-gradient(180deg, #02050a 0%, #07111b 100%);
        color: #e6eefb;
      }}
      .wrap {{
        width: 1500px;
        margin: 0 auto;
        padding: 42px;
      }}
      .hero {{
        border: 1px solid rgba(255,255,255,0.08);
        background: linear-gradient(180deg, rgba(10,17,28,0.96), rgba(6,11,19,0.96));
        border-radius: 28px;
        overflow: hidden;
        box-shadow: 0 28px 60px rgba(0,0,0,0.34);
      }}
      .top {{
        padding: 26px 30px 18px;
        border-bottom: 1px solid rgba(255,255,255,0.06);
      }}
      .eyebrow {{
        color: #7fd0ff;
        font-size: 11px;
        font-weight: 900;
        letter-spacing: .28em;
        text-transform: uppercase;
      }}
      h1 {{
        margin: 16px 0 0;
        font-size: 58px;
        line-height: .94;
        font-family: Georgia, "Times New Roman", serif;
        letter-spacing: -.05em;
      }}
      p {{
        margin: 14px 0 0;
        color: #9cb1d0;
        font-size: 18px;
        line-height: 1.55;
        max-width: 980px;
      }}
      .terminal {{
        margin: 26px 30px 30px;
        border-radius: 22px;
        border: 1px solid rgba(255,255,255,0.08);
        background: rgba(2,6,12,0.92);
        overflow: hidden;
      }}
      .terminal-head {{
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 16px 18px;
        border-bottom: 1px solid rgba(255,255,255,0.08);
        background: rgba(255,255,255,0.03);
      }}
      .lights {{
        display: flex;
        gap: 8px;
      }}
      .lights i {{
        width: 10px;
        height: 10px;
        border-radius: 999px;
        display: block;
      }}
      .lights i:nth-child(1) {{ background: rgba(248,113,113,0.82); }}
      .lights i:nth-child(2) {{ background: rgba(251,191,36,0.82); }}
      .lights i:nth-child(3) {{ background: rgba(74,222,128,0.82); }}
      .cmd {{
        color: #7fd0ff;
        font-size: 11px;
        font-weight: 800;
        letter-spacing: .14em;
        text-transform: uppercase;
      }}
      pre {{
        margin: 0;
        padding: 22px;
        white-space: pre-wrap;
        color: #dbeafe;
        font-size: 15px;
        line-height: 1.7;
        font-family: "Cascadia Code", Consolas, monospace;
      }}
    </style>
  </head>
  <body>
    <div class="wrap">
      <section class="hero">
        <div class="top">
          <div class="eyebrow">AEO Linter</div>
          <h1>{html.escape(title)}</h1>
          <p>{html.escape(subtitle)}</p>
        </div>
        <div class="terminal">
          <div class="terminal-head">
            <div class="lights"><i></i><i></i><i></i></div>
            <div class="cmd">{html.escape(command)}</div>
          </div>
          <pre>{html.escape(output)}</pre>
        </div>
      </section>
    </div>
  </body>
</html>""",
        encoding="utf-8",
    )
    return page


def capture(page: Path) -> None:
    edge = edge_path()
    if edge is None:
        return
    subprocess.run(
        [
            str(edge),
            "--headless",
            "--disable-gpu",
            "--hide-scrollbars",
            "--window-size=1600,980",
            f"--screenshot={page.with_suffix('.png')}",
            page.as_uri(),
        ],
        check=True,
    )


def main() -> None:
    for path in OUT.glob("*"):
        if path.is_file():
            path.unlink()

    scenes = [
        (
            "01-ready-manifest",
            "Ready manifest with clean answer-engine posture.",
            "AEO manifests should be able to pass with clear source coverage, strong entity linkage, and explicit AI citation policy.",
            "cargo run -- lint .\\samples\\manifest-ready.json",
            *run_cli(["lint", ".\\samples\\manifest-ready.json"]),
        ),
        (
            "02-problem-manifest",
            "Problem manifest with blocking citation and policy gaps.",
            "The linter should fail loudly when claims, identity, freshness, and policy posture are too weak for trustworthy reuse.",
            "cargo run -- lint .\\samples\\manifest-problem.json",
            *run_cli(["lint", ".\\samples\\manifest-problem.json"]),
        ),
        (
            "03-json-findings",
            "Machine-readable lint findings for CI and automation.",
            "Teams often need JSON output that can flow into pipelines, dashboards, or compliance review steps.",
            "cargo run -- lint .\\samples\\manifest-problem.json --format json",
            *run_cli(["lint", ".\\samples\\manifest-problem.json", "--format", "json"]),
        ),
        (
            "04-rule-set",
            "Rule categories explained in operator language.",
            "The explain mode keeps the CLI legible for people who need the intent behind each lint category.",
            "cargo run -- explain",
            *run_cli(["explain"]),
        ),
    ]

    for name, title, subtitle, command, output, code in scenes:
        page = write_scene(name, title, subtitle, command + f"  (exit {code})", output)
        capture(page)

    print("rendered screenshots")


if __name__ == "__main__":
    main()
