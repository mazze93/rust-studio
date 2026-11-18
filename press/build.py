#!/usr/bin/env python3
"""Build the Rustpress corpus edition of The Rust Field Guide.

Reads the Astro chapters from ../site/src/chapters, substitutes the data that
studio-verify recorded (site/src/data/verified.json), and emits a static,
JavaScript-free corpus that rustpress's content policy accepts:

  out/index.html   the whole guide, one page, one stylesheet link
  out/field-guide.css

Nothing here is hand-maintained HTML; the chapters remain the single source of
truth. Re-run after any prose or example change, then press a new release.
"""
import html
import json
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
SITE = HERE.parent / "site" / "src"
OUT = HERE / "out"

verified = json.loads((SITE / "data" / "verified.json").read_text())
sources = json.loads((SITE / "data" / "sources.json").read_text())
examples = {e["name"]: e for e in verified["examples"]}

CHAPTERS = [
    ("01-start", "start", "01 · Start"),
    ("02-current", "current", "02 · Current"),
    ("03-syntax", "syntax", "03 · Syntax"),
    ("04-ownership", "ownership", "04 · Ownership"),
    ("05-lifetimes", "lifetimes", "05 · Lifetimes"),
    ("06-model", "model", "06 · Model"),
    ("07-errors", "errors", "07 · Errors"),
    ("08-collections", "collections", "08 · Collections"),
    ("09-abstraction", "abstraction", "09 · Abstraction"),
    ("10-concurrency", "concurrency", "10 · Concurrency"),
    ("11-workflow", "workflow", "11 · Workflow"),
    ("12-judgment", "judgment", "12 · Judgment"),
    ("13-practice", "practice", "13 · Practice"),
    ("14-maintenance", "maintenance", "14 · Maintenance"),
    ("15-sources", "sources", "15 · Sources"),
]

LAB_CASES = [
    ("move", "Move a String, then use the old name",
     "The String moved to second. Using first afterward is rejected. Passing ownership does not implicitly clone the allocation."),
    ("borrow", "Borrow the String instead",
     "second borrows first. Both names can be used for this shared read, and first remains the owner."),
    ("conflict", "Mutate before a shared borrow's final use",
     "view is used after push. The shared borrow therefore conflicts with mutable access during push."),
    ("last_use", "Mutate after the borrow's final use",
     "The final use of view is before push. The compiler can end that borrow, permitting the subsequent mutation."),
    ("clone", "Make a deliberate independent clone",
     "clone creates an independent String. Mutation of second does not change first; copying the text has a cost."),
    ("copy", "Copy an integer",
     "u32 implements Copy. The assignment copies its value; no String allocation is involved."),
]


def esc(text: str) -> str:
    return html.escape(text, quote=True)


def render_example(name: str) -> str:
    ex = examples[name]
    ok = ex["expect"] == "pass"
    toolchain = f"rustc {verified['toolchain']}"
    verdict = (
        f"compiles and runs on {toolchain}"
        if ok
        else f"rejected with {ex['expect']} by {toolchain}"
    )
    out = (ex["stdout"] if ok else ex["stderr"]).strip("\n")
    label = "Program output" if ok else f"Compiler diagnostic ({ex['expect']})"
    return (
        f'<figure class="example {"pass" if ok else "fail"}">'
        f'<figcaption><span class="ex-path">examples/{esc(name)}.rs</span>'
        f'<span class="ex-verdict">{esc(verdict)}</span></figcaption>'
        f"<pre><code>{esc(ex['source'].rstrip())}</code></pre>"
        f'<details class="output"><summary>{esc(label)}</summary><pre>{esc(out)}</pre></details>'
        f"</figure>"
    )


def render_code(label: str, body: str) -> str:
    return f'<div class="codebox"><div class="codehead"><span>{esc(label)}</span></div><pre><code>{esc(body)}</code></pre></div>'


def render_lab() -> str:
    parts = [
        '<div class="lab"><h3>The six experiments, predict first</h3>',
        "<p>Decide for each one: does it compile? Open the verdict only after you have called it.</p>",
    ]
    for i, (name, title, explain) in enumerate(LAB_CASES, 1):
        ex = examples[name]
        ok = ex["expect"] == "pass"
        out = (ex["stdout"] if ok else ex["stderr"]).strip("\n")
        verdict = "It compiles and runs." if ok else f"The compiler rejects it: {ex['expect']}."
        parts.append(
            f'<details class="experiment"><summary><span class="ex-index">{i:02}</span> {esc(title)}</summary>'
            f"<p>{esc(explain)}</p>"
            f"<p><strong>{esc(verdict)}</strong></p>"
            f"<pre><code>{esc(out)}</code></pre>"
            f"</details>"
        )
    parts.append("</div>")
    return "".join(parts)


def render_sources() -> str:
    items = []
    for s in sources:
        items.append(
            f'<li id="src-{esc(s["id"])}"><a href="{esc(s["url"])}">{esc(s["title"])}</a>'
            f'<p>{esc(s["note"])} <span class="accessed">Consulted {esc(s["accessed"])}.</span></p></li>'
        )
    return '<ol class="sources">\n' + "\n".join(items) + "\n</ol>"


def chapter_body(name: str) -> str:
    text = (SITE / "chapters" / f"{name}.astro").read_text()
    text = re.sub(r"\A---\n.*?\n---\n", "", text, flags=re.S)
    text = text.replace(
        'href="/starter/"',
        'href="https://github.com/mazze93/rust-studio/tree/main/starter"',
    )
    # data substitutions
    text = text.replace("{verified.toolchain}", verified["toolchain"])
    text = text.replace("{verified.edition}", verified["edition"])
    text = text.replace("{verified.rustc}", verified["rustc"])
    text = text.replace("{verified.starter.stdout.trim()}", verified["starter"]["stdout"].strip())
    text = re.sub(r'<ol class="sources">.*?</ol>', render_sources(), text, flags=re.S)
    leftover = re.findall(r"\{[A-Za-z][^}]*\}", text)
    if leftover:
        sys.exit(f"unresolved template expression(s) in {name}: {leftover}")
    text = re.sub(r"<Example name=\"([a-z_]+)\" />", lambda m: render_example(m.group(1)), text)
    text = re.sub(
        r'<Code label="([^"]*)" code=\{"((?:[^"\\]|\\.)*)"\} />',
        lambda m: render_code(m.group(1), json.loads('"' + m.group(2) + '"')),
        text,
    )
    text = text.replace("<OwnershipLab />", render_lab())
    text = re.sub(r'<input type="checkbox"[^>]*>', "", text)
    text = text.replace("<label>", '<div class="exercise">').replace("</label>", "</div>")
    text = re.sub(r'<p id="progress-status"[^>]*>.*?</p>\n?', "", text, flags=re.S)
    text = re.sub(
        r"<p>Progress is stored only in this browser[^<]*</p>",
        "<p>Tick these off on paper or in your editor; nothing on this page tracks you.</p>",
        text,
    )
    return text.strip()


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    sections = "\n".join(chapter_body(name) for name, _, _ in CHAPTERS)
    nav = "\n".join(
        f'<a href="#{anchor}">{label}</a>' for _, anchor, label in CHAPTERS
    )
    passed = sum(1 for e in verified["examples"] if e["expect"] == "pass")
    failed = len(verified["examples"]) - passed
    template = (HERE / "template.html").read_text()
    out = (
        template
        .replace("{{NAV}}", nav)
        .replace("{{SECTIONS}}", sections)
        .replace("{{TOOLCHAIN}}", verified["toolchain"])
        .replace("{{EDITION}}", verified["edition"])
        .replace("{{RUSTC}}", verified["rustc"])
        .replace("{{PASS_COUNT}}", str(passed))
        .replace("{{FAIL_COUNT}}", str(failed))
        .replace("{{EXAMPLE_COUNT}}", str(len(verified["examples"])))
        .replace("{{SOURCE_COUNT}}", str(len(sources)))
    )
    frontmatter = (
        "---\n"
        "title: The Rust Field Guide\n"
        "description: A practical, source-linked Rust primer for stable 1.100.0 and edition 2024. Every example was compiled by the pinned compiler at build time.\n"
        'date: "2026-11-18"\n'
        "tags: [rust, field-guide]\n"
        "draft: false\n"
        "---\n"
    )
    (OUT / "index.html").write_text(frontmatter + out)
    (OUT / "field-guide.css").write_text((HERE / "style.css").read_text())
    print(f"wrote {OUT/'index.html'} and field-guide.css")


if __name__ == "__main__":
    main()
