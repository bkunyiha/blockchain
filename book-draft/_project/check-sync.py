#!/usr/bin/env python3
"""
check-sync.py — Verify LaTeX and Markdown sources are content-synchronized.

Run from the book root:  python3 book-draft/_project/check-sync.py

Compares key content markers between LaTeX chapters and their corresponding
markdown files. Reports mismatches that indicate one source was updated
without the other.
"""
import re, os, sys

BOOK_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
LATEX_DIR = os.path.join(BOOK_ROOT, 'book-latex', 'chapters')
MD_DIR = os.path.join(BOOK_ROOT, 'book-draft')

# LaTeX file -> markdown file(s) mapping
CHAPTER_MAP = {
    'ch00-quickstart.tex': ['00-Quick-Start.md'],
    'ch01-introduction.tex': ['01-Introduction.md'],
    'ch02-blockchain-intro.tex': ['bitcoin-blockchain/README.md'],
    'ch03-whitepaper.tex': ['bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md'],
    'ch04-whitepaper-rust.tex': ['bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md'],
    'ch05-project-index.tex': ['bitcoin-blockchain/Rust-Project-Index.md'],
    'ch06-primitives.tex': ['bitcoin-blockchain/primitives/README.md'],
    'ch07-utilities.tex': ['bitcoin-blockchain/util/README.md'],
    'ch08-cryptography.tex': ['bitcoin-blockchain/crypto/README.md'],
    'ch09-1-domain-model.tex': ['bitcoin-blockchain/chain/01-Domain-Model.md'],
    'ch09-2-state-management.tex': ['bitcoin-blockchain/chain/02-Blockchain-State-Management.md'],
    'ch09-3-chain-state.tex': ['bitcoin-blockchain/chain/03-Chain-State-and-Storage.md'],
    'ch09-4-utxo-set.tex': ['bitcoin-blockchain/chain/04-UTXO-Set.md'],
    'ch09-5-transaction-lifecycle.tex': ['bitcoin-blockchain/chain/05-Transaction-Lifecycle.md'],
    'ch09-6-block-lifecycle.tex': ['bitcoin-blockchain/chain/06-Block-Lifecycle-and-Mining.md'],
    'ch09-7-consensus.tex': ['bitcoin-blockchain/chain/07-Consensus-and-Validation.md'],
    'ch09-8-node-orchestration.tex': ['bitcoin-blockchain/chain/08-Node-Orchestration.md'],
    'ch09-9-tx-to-block.tex': ['bitcoin-blockchain/chain/09-Transaction-to-Block.md'],
    'ch10-block-acceptance.tex': ['bitcoin-blockchain/chain/10-Block-Acceptance.md'],
    'ch11-storage.tex': ['bitcoin-blockchain/store/README.md'],
    'ch12-network.tex': ['bitcoin-blockchain/net/README.md'],
    'ch13-node-orchestration.tex': ['bitcoin-blockchain/node/README.md'],
    'ch14-wallet.tex': ['bitcoin-blockchain/wallet/README.md'],
    'ch15-web-api.tex': ['bitcoin-blockchain/web/README.md'],
    'ch16-desktop-iced.tex': ['bitcoin-desktop-ui-iced/README.md'],
    'ch17-desktop-tauri.tex': ['bitcoin-desktop-ui-tauri/README.md'],
    'ch18-wallet-iced.tex': ['bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md'],
    'ch19-wallet-tauri.tex': ['bitcoin-wallet-ui-tauri/README.md'],
    'ch20-embedded-db.tex': ['embedded-database/06-Embedded-Database.md'],
    'ch21-web-ui.tex': ['bitcoin-web-ui/06-Web-Admin-UI.md'],
    'ch22-docker.tex': ['ci/docker-compose/01-Introduction.md'],
    'ch23-kubernetes.tex': ['ci/kubernetes/README.md'],
    'ch24-rust-guide.tex': ['rust/README.md'],
}

def strip_latex(text):
    """Extract plain text from LaTeX."""
    text = re.sub(r'%.*$', '', text, flags=re.MULTILINE)
    text = re.sub(r'\\(textbf|textit|emph|code|index|textbf)\{([^}]*)\}', r'\2', text)
    text = re.sub(r'\\(begin|end)\{[^}]*\}(\[[^\]]*\])?', '', text)
    text = re.sub(r'\\(section|subsection|chapter|label|caption|ref|cref|url)\{[^}]*\}', '', text)
    text = re.sub(r'\\_', '_', text)
    text = re.sub(r'\\[a-zA-Z]+', ' ', text)
    text = re.sub(r'[{}~]', ' ', text)
    return re.sub(r'\s+', ' ', text).strip().lower()

def strip_markdown(text):
    """Extract plain text from Markdown."""
    # Remove nav blocks
    text = re.sub(r'<details>.*?</details>', '', text, flags=re.DOTALL)
    text = re.sub(r'<[^>]+>', '', text)
    text = re.sub(r'^#+\s+', '', text, flags=re.MULTILINE)
    text = re.sub(r'\*\*([^*]+)\*\*', r'\1', text)
    text = re.sub(r'`([^`]+)`', r'\1', text)
    text = re.sub(r'\[([^\]]+)\]\([^)]+\)', r'\1', text)
    return re.sub(r'\s+', ' ', text).strip().lower()

def extract_sentences(text, min_len=40):
    """Extract sentences long enough to be meaningful content."""
    # Split on periods followed by space or end
    sentences = re.split(r'(?<=[.!?])\s+', text)
    return [s.strip() for s in sentences if len(s.strip()) >= min_len]

def main():
    print("=" * 65)
    print("  SYNC CHECK: LaTeX ↔ Markdown Content Comparison")
    print("=" * 65)
    
    issues = []
    ok_count = 0
    
    for tex_file, md_files in sorted(CHAPTER_MAP.items()):
        tex_path = os.path.join(LATEX_DIR, tex_file)
        if not os.path.exists(tex_path):
            continue
        
        with open(tex_path, 'r') as f:
            tex_raw = f.read()
        
        md_raw = ""
        md_found = []
        for md_file in md_files:
            md_path = os.path.join(MD_DIR, md_file)
            if os.path.exists(md_path):
                with open(md_path, 'r') as f:
                    md_raw += f.read()
                md_found.append(md_file)
        
        if not md_raw:
            issues.append((tex_file, "NO MARKDOWN FILE FOUND", md_files))
            continue
        
        tex_text = strip_latex(tex_raw)
        md_text = strip_markdown(md_raw)
        
        # Check for important content terms in one but not the other
        content_terms = set()
        # Extract unique multi-word terms from LaTeX
        for match in re.finditer(r'\\(?:textbf|code)\{([^}]+)\}', tex_raw):
            term = match.group(1).replace('\\_', '_').lower().strip()
            if len(term) > 5:
                content_terms.add(term)
        
        term_issues = []
        for term in content_terms:
            if term in tex_text and term not in md_text:
                term_issues.append(f"    '{term}' in LaTeX but NOT in Markdown")
        
        # Check reverse: important markdown content missing from LaTeX
        for match in re.finditer(r'\*\*([^*]+)\*\*', md_raw):
            term = match.group(1).lower().strip()
            if len(term) > 5 and term in md_text.lower() and term not in tex_text:
                term_issues.append(f"    '{term[:50]}' in Markdown but NOT in LaTeX")
        
        # Deduplicate
        term_issues = list(set(term_issues))[:5]  # Cap at 5
        
        if term_issues:
            issues.append((tex_file, "CONTENT MISMATCH", term_issues))
        else:
            ok_count += 1
    
    # Print results
    print(f"\n✅ {ok_count} chapters: content appears synchronized")
    
    if issues:
        print(f"\n⚠️  {len(issues)} chapters with potential differences:\n")
        for tex_file, issue_type, details in issues:
            print(f"  {tex_file}: {issue_type}")
            if isinstance(details, list):
                for d in details[:5]:
                    if isinstance(d, str):
                        print(f"  {d}")
            print()
    else:
        print("\nAll chapters appear to be in sync!")
    
    print("=" * 65)
    print("NOTE: This checks key content terms. For a full comparison,")
    print("read both formats side-by-side for chapters you've edited.")
    print("=" * 65)
    
    return 1 if issues else 0

if __name__ == '__main__':
    sys.exit(main())
