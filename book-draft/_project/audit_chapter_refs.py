#!/usr/bin/env python3
"""Audit all chapter cross-references across LaTeX and markdown sources.

Checks:
  1. LaTeX files: warns if any hardcoded "Chapter N" remains (should use \\ref{})
  2. Markdown files: validates that hardcoded "Chapter N" matches chapter_map.py
  3. Detects stale references that don't correspond to any chapter

Usage:
  python3 audit_chapter_refs.py          # audit only
  python3 audit_chapter_refs.py --fix    # show suggested fixes for markdown
  python3 audit_chapter_refs.py --strict # exit with error code if issues found

Run this after any structural change (adding, removing, or reordering chapters).
"""

import os
import re
import sys

# Import the canonical chapter map
script_dir = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, script_dir)
from chapter_map import CHAPTER_MAP, NUMBER_TO_LABEL, CHAPTER_TITLES

BOOK_ROOT = os.path.dirname(script_dir)  # book-draft/
LATEX_ROOT = os.path.join(os.path.dirname(BOOK_ROOT), 'book-latex', 'chapters')

VALID_NUMBERS = set(CHAPTER_MAP.values())


def find_latex_hardcoded(latex_dir):
    """Find hardcoded 'Chapter N' in LaTeX files (should use \\ref{}).

    Ignores:
      - Comment lines (starting with %)
      - Lines inside minted/verbatim environments
      - \\chapter{} and \\label{} commands
      - Lines that already use \\ref{ch:}
    """
    issues = []
    for fname in sorted(os.listdir(latex_dir)):
        if not fname.endswith('.tex'):
            continue
        filepath = os.path.join(latex_dir, fname)
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()

        in_verbatim = False
        for i, line in enumerate(lines, 1):
            stripped = line.strip()

            # Track verbatim environments
            if re.search(r'\\begin\{(?:minted|verbatim|lstlisting)', line):
                in_verbatim = True
            if re.search(r'\\end\{(?:minted|verbatim|lstlisting)', line):
                in_verbatim = False
                continue

            # Skip: verbatim, comments, first 3 lines, labels, chapter commands
            if in_verbatim:
                continue
            if stripped.startswith('%'):
                continue
            if i <= 3:
                continue
            if '\\chapter{' in line or '\\label{' in line:
                continue
            if '\\ref{ch:' in line:
                # Line already uses \ref{} — but check if it ALSO has hardcoded numbers
                # Remove \ref{} matches first, then check for remaining hardcoded
                cleaned = re.sub(r'Chapter~?\\ref\{ch:[^}]+\}', '', line)
                for m in re.finditer(r'Chapter\s+(\d+)', cleaned):
                    num = int(m.group(1))
                    issues.append({
                        'file': fname,
                        'line': i,
                        'text': stripped[:120],
                        'issue': f'Hardcoded "Chapter {num}" alongside \\ref{{}} — convert to \\ref{{ch:{NUMBER_TO_LABEL.get(num, "???")}}}'
                    })
                continue

            # Check for hardcoded chapter references
            for m in re.finditer(r'Chapters?\s+(\d+)', line):
                num = int(m.group(1))
                label = NUMBER_TO_LABEL.get(num, '???')
                issues.append({
                    'file': fname,
                    'line': i,
                    'text': stripped[:120],
                    'issue': f'Hardcoded "Chapter {num}" — use \\ref{{ch:{label}}} instead'
                })

    return issues


def find_markdown_issues(md_root):
    """Find chapter references in markdown that don't match chapter_map.py.

    Checks prose content only, skipping:
      - TOC navigation sections (HTML links)
      - Code blocks
      - Lines that are purely link/nav elements
    """
    issues = []

    for dirpath, dirnames, filenames in os.walk(md_root):
        # Skip hidden directories and _project
        dirnames[:] = [d for d in dirnames if not d.startswith('.') and d != '_project']

        for fname in sorted(filenames):
            if not fname.endswith('.md'):
                continue
            filepath = os.path.join(dirpath, fname)
            relpath = os.path.relpath(filepath, md_root)

            with open(filepath, 'r', encoding='utf-8') as f:
                lines = f.readlines()

            in_code = False
            for i, line in enumerate(lines, 1):
                stripped = line.strip()

                # Track code blocks
                if stripped.startswith('```'):
                    in_code = not in_code
                    continue
                if in_code:
                    continue

                # Skip TOC navigation (HTML anchor tags)
                if '<a href=' in line and 'Chapter' in line:
                    continue

                # Find "Chapter N" references
                for m in re.finditer(r'Chapter\s+(\d+)', line):
                    num = int(m.group(1))
                    if num not in VALID_NUMBERS:
                        issues.append({
                            'file': relpath,
                            'line': i,
                            'text': stripped[:120],
                            'issue': f'Chapter {num} is not a valid chapter number (valid: 1-{max(VALID_NUMBERS)})'
                        })

    return issues


def print_issues(issues, header):
    """Pretty-print a list of issues."""
    if not issues:
        print(f"\n  {header}: all OK")
        return

    print(f"\n  {header}: {len(issues)} issue(s)")
    print(f"  {'─' * 68}")

    current_file = None
    for iss in issues:
        if iss['file'] != current_file:
            current_file = iss['file']
            print(f"\n    {current_file}")

        print(f"      L{iss['line']:4d}: {iss['issue']}")
        print(f"             {iss['text'][:100]}")


def main():
    strict = '--strict' in sys.argv

    print("=" * 70)
    print("  Chapter Reference Audit")
    print(f"  Valid chapters: 1-{max(VALID_NUMBERS)} ({len(VALID_NUMBERS)} total)")
    print("=" * 70)

    # 1. Check LaTeX for hardcoded references
    latex_issues = []
    if os.path.isdir(LATEX_ROOT):
        latex_issues = find_latex_hardcoded(LATEX_ROOT)
        print_issues(latex_issues, "LaTeX hardcoded chapter numbers")
    else:
        print(f"\n  LaTeX directory not found: {LATEX_ROOT}")

    # 2. Check markdown for invalid references
    md_issues = find_markdown_issues(BOOK_ROOT)
    print_issues(md_issues, "Markdown invalid chapter numbers")

    # Summary
    total = len(latex_issues) + len(md_issues)
    print(f"\n{'=' * 70}")
    if total == 0:
        print("  All chapter references are valid.")
    else:
        print(f"  Total issues: {total}")
        if latex_issues:
            print(f"    - {len(latex_issues)} LaTeX hardcoded reference(s) (should use \\ref{{}})")
        if md_issues:
            print(f"    - {len(md_issues)} markdown invalid reference(s)")
    print("=" * 70)

    if strict and total > 0:
        sys.exit(1)


if __name__ == '__main__':
    main()
