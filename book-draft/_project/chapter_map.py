#!/usr/bin/env python3
"""Single source of truth for chapter numbering.

This file maps symbolic chapter labels to their printed chapter numbers.
LaTeX resolves these automatically via \\label/\\ref; this map is used by:
  - gen_epub.py      (validates markdown references during EPUB build)
  - audit_chapter_refs.py  (catches stale hardcoded numbers in any file)

If chapters are added, removed, or reordered, update THIS file first,
then run audit_chapter_refs.py to find and fix all stale references.
"""

# Symbolic label -> printed chapter number
# Labels match the LaTeX \label{ch:...} definitions in book-latex/chapters/
CHAPTER_MAP = {
    'quickstart':               1,
    'introduction':             2,
    'blockchain-intro':         3,
    'whitepaper':               4,
    'whitepaper-rust':          5,
    'project-index':            6,
    'primitives':               7,
    'utilities':                8,
    'cryptography':             9,
    'domain-model':            10,
    'blockchain-state-management': 11,
    'chain-state':             12,
    'utxo-set':                13,
    'transaction-lifecycle':   14,
    'block-lifecycle':         15,
    'consensus':               16,
    'chain-node-orchestration': 17,
    'tx-to-block':             18,
    'block-acceptance':        19,
    'storage':                 20,
    'network':                 21,
    'node-orchestration':      22,
    'wallet':                  23,
    'web-api':                 24,
    'desktop-iced':            25,
    'desktop-tauri':           26,
    'wallet-iced':             27,
    'wallet-tauri':            28,
    'embedded-db':             29,
    'web-admin-ui':            30,
    'docker-deployment':       31,
    'k8s-deployment':          32,
    'rust-guide':              33,
}

# Reverse map: printed number -> symbolic label
NUMBER_TO_LABEL = {v: k for k, v in CHAPTER_MAP.items()}

# Human-readable chapter titles (for audit script output)
CHAPTER_TITLES = {
    1:  'Quick Start',
    2:  'Introduction & Overview',
    3:  'Introduction to Blockchain',
    4:  'Bitcoin Whitepaper',
    5:  'Bitcoin Whitepaper in Rust',
    6:  'Rust Blockchain Project',
    7:  'Primitives',
    8:  'Utilities',
    9:  'Cryptography',
    10: 'Domain Model',
    11: 'Blockchain State Management',
    12: 'Chain State and Storage',
    13: 'UTXO Set',
    14: 'Transaction Lifecycle',
    15: 'Block Lifecycle and Mining',
    16: 'Consensus and Validation',
    17: 'Node Orchestration (Chain)',
    18: 'Transaction to Block',
    19: 'Block Acceptance',
    20: 'Storage Layer',
    21: 'Network Layer',
    22: 'Node Orchestration',
    23: 'Wallet System',
    24: 'Web API Architecture',
    25: 'Desktop Admin (Iced)',
    26: 'Desktop Admin (Tauri)',
    27: 'Wallet UI (Iced)',
    28: 'Wallet UI (Tauri)',
    29: 'Embedded Database',
    30: 'Web Admin Interface',
    31: 'Docker Compose Deployment',
    32: 'Kubernetes Deployment',
    33: 'Rust Language Guide',
}


if __name__ == '__main__':
    print(f"Total chapters: {len(CHAPTER_MAP)}")
    print(f"\nChapter mapping:")
    for label, num in sorted(CHAPTER_MAP.items(), key=lambda x: x[1]):
        title = CHAPTER_TITLES.get(num, '')
        print(f"  Chapter {num:2d} = ch:{label:<30s} ({title})")
