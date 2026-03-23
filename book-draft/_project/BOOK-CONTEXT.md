# Book Context & Editorial Guide

> **Purpose**: Read this file at the start of every editing session. It contains the style conventions, chapter status, known issues, and roadmap for the book. Keep it updated as work progresses.

> **IMPORTANT — Ask, Don't Assume**: When working on this book, **never assume answers to unclear or ambiguous questions**. If something is uncertain — scope of a task, whether a chapter should be changed, how to handle a discrepancy, naming conventions, structural decisions, etc. — **stop and ask the user for clarification before proceeding**. Log any open questions in the [Open Questions](#open-questions) section below. The user prefers to make decisions collaboratively rather than have choices made silently on their behalf.

---

## Project Overview

**Title**: Rust Blockchain: A Full-Stack Implementation Guide
**Audience**: Intermediate to advanced Rust developers
**Format**: Pure markdown (119+ files), no build system (no mdBook)
**Location**: `/book-draft/` within the `blockchain` project root
**Codebase**: Rust workspace at project root with these key crates/apps:

### Core Principle: Self-Contained Code Walkthrough

> **This book is primarily a detailed code walkthrough.** The reader should be able to fully understand the entire system — every module, every function, every design decision — without ever cloning the repository or opening the code in an editor.
>
> This means:
> - **Copy all relevant code into the book.** Every struct, function, impl block, and configuration file that matters should appear in the text. Do not say "see the source" or "refer to the repo."
> - **Explain every piece of code in detail.** Line-by-line or block-by-block annotations. What does it do? Why was it written this way? What alternatives were considered?
> - **Show complete logical units.** Never show a fragment and expect the reader to infer the rest. If a function calls another function, show both.
> - **Provide context before code.** Before each code block, explain what problem it solves and where it fits in the architecture. After each code block, explain the key takeaways.
> - **Companion chapters excluded from print.** The 15 companion chapters (A/B/C suffix files) contain verbatim source listings totalling 17,315 lines. These are replaced in the print edition by `Appendix-Source-Reference.md` — annotated directory trees with file-level descriptions and a repository link with QR code. The companion files remain in the markdown source for digital readers.
> - **The book IS the documentation.** A reader who finishes this book should understand the codebase as well as someone who wrote it.

### Exception: Bitcoin Whitepaper Chapters (3 & 4)

> The chapters in `book-draft/bitcoin-blockchain/whitepaper-rust/` (Chapters 3 and 4) are **conceptual implementation guides**, not verbatim code walkthroughs. They show *how one could implement* the Bitcoin whitepaper in Rust — explaining the mapping from whitepaper concepts (transactions, proof-of-work, timestamps, etc.) to Rust data structures and algorithms. The code in these chapters is a **sample implementation** that illustrates the ideas; it does not necessarily correspond line-for-line to the actual project codebase. During the technical accuracy pass (Phase 2), these chapters should be checked for conceptual correctness and internal consistency, but **not** compared against specific source files in the repository.

| Component | Path | Tech Stack |
|-----------|------|------------|
| Blockchain core | `bitcoin/` | Rust (tokio, serde, rusqlite) |
| API server | `bitcoin-api/` | Rust (shared crate used by all clients) |
| Web UI | `bitcoin-web-ui/` | React, TypeScript, Vite |
| Desktop Admin (Iced) | `bitcoin-desktop-ui-iced/` | Rust, Iced framework |
| Wallet (Iced) | `bitcoin-wallet-ui-iced/` | Rust, Iced framework |
| Desktop Admin (Tauri) | `bitcoin-desktop-ui-tauri/` | Rust backend + React/TS frontend |
| Wallet (Tauri) | `bitcoin-wallet-ui-tauri/` | Rust backend + React/TS frontend, SQLCipher |
| Docker Compose | `ci/docker-compose/` | YAML configs |
| Kubernetes | `ci/kubernetes/` | K8s manifests |

---

## Style Guide & Conventions

### Tone & Voice
- Use **"we"** language ("we keep", "we avoid", "we want") — confident first-person plural
- Senior-engineer-at-a-whiteboard register: direct, technically precise, no filler
- Technical but pedagogical: explain the *why* behind design decisions, not just the *what*
- Assume the reader knows Rust but not necessarily the specific frameworks (Iced, Tauri, Axum, etc.)
- Pragmatic, not academic. Concrete examples over abstract theory
- **No blog patterns**: no emoji, no enthusiasm markers ("Let's get started!"), no self-referential meta-commentary ("This chapter has introduced..."), no contract paragraphs about the reading experience, no syllabus previews ("Our Learning Journey"), no "In one sentence" hooks
- Prefer prose over bullet lists for introductory/connective material; reserve bullets for genuinely list-shaped content (e.g., file inventories, step sequences)

### Chapter Structure (Standard Template)
Each main chapter follows this pattern:
1. **Navigation box** (expandable `<details>` with full chapter listing, "You are here" marker)
2. **Breadcrumb navigation** at top: `[← Previous] | Current | [Next →]`
3. **One-sentence overview** of what the component is
4. **Mental model / why it exists** (paragraph)
5. **Architecture diagram** (ASCII or Mermaid)
6. **Sections with annotated code** (complete methods, not fragments)
7. **Walkthrough** explaining design intent and patterns
8. **Breadcrumb navigation** at bottom

### Code Presentation
- **Main chapters**: Full code blocks with detailed prose explanations before and after. Inline comments explain "why," surrounding prose explains "what" and "how it fits"
- **Companion chapters** (suffixed A, B, C): Excluded from print edition. Replaced by `Appendix-Source-Reference.md` with annotated directory trees and repository link. Digital/markdown readers can still access the full verbatim listings
- Show **complete methods, structs, and impl blocks** — never fragments. If a function is 50 lines, show all 50 lines
- If function A calls function B, show both and explain the call chain
- Include **Cargo.toml dependencies**, **module declarations** (`mod.rs`), and **imports** — these are part of understanding the code
- Use `rust` syntax highlighting for Rust, `typescript` for TS, `tsx` for React components, `bash` for shell, `toml` for Cargo files, `json` for config
- Every code block should be preceded by a sentence or paragraph explaining what the reader is about to see and why it matters

### Visual Elements
- **No emoji** — all emoji removed from prose and navigation (preserved only in code listings where they appear as literal UI text)
- Bold and blockquotes for emphasis
- `---` dividers between major sections
- `<details>/<summary>` for collapsible navigation
- `<div align="center">` for centered elements

### Naming Conventions
- Chapter files: `NN-Title-With-Dashes.md` (e.g., `03-Desktop-Admin-UI.md`)
- Companion code files: `NNA-Title-Code-Listings.md` (e.g., `03A-Boot-Runtime-Types.md`)
- README.md in each directory serves as the chapter index/entry point

---

## Chapter Status Tracker

### Part I: Foundations & Core Implementation

| # | Chapter | Status | Notes |
|---|---------|--------|-------|
| 1 | Introduction & Overview | **PHASE 2 DONE** | Verified |
| 2 | Introduction to Blockchain | **PHASE 2 DONE** | Verified |
| 3 | Bitcoin Whitepaper Summary | **PHASE 2 DONE** | Conceptual check — all code syntactically valid |
| 4 | Bitcoin Whitepaper In Rust | **PHASE 2 DONE** | Conceptual check — 17 sub-files + appendices all consistent |
| 5 | Rust Blockchain Project | **PHASE 2 DONE** | Verified |
| 6 | Primitives | **PHASE 2 DONE** | 3 fixes: Block::new_block params, Transaction::new_utxo_transaction sig, Blockchain ops |
| 7 | Utilities | **PHASE 2 DONE** | Verified |
| 8 | Cryptography | **PHASE 2 DONE** | 6 sub-files verified |
| 9 | Chain (Code Walkthrough) | **PHASE 2 DONE** | 11 sub-files verified |
| 10 | Block Acceptance | **PHASE 2 DONE** | Verified |
| 11 | Storage Layer | **PHASE 2 DONE** | Verified |
| 12 | Network Layer | **PHASE 2 DONE** | 3 sub-files verified |
| 13 | Node Orchestration | **PHASE 2 DONE** | Verified |
| 14 | Wallet System | **PHASE 2 DONE** | Verified |
| 15 | Web API Architecture | **PHASE 2 DONE** | 5 fixes: create_app() return type, BalanceResponse fields. 14 sub-files verified |
| 16 | Desktop Admin (Iced) | **PHASE 2 DONE** | 1 fix: runtime.rs comments in 16A. 4 files verified |
| 17 | **Desktop Admin (Tauri)** | **COMPLETE** | Written from source + spot-checked. 4 files. |
| 18 | Wallet UI (Iced) | **PHASE 2 DONE** | All verbatim code verified. 2 files. |
| 19 | **Wallet UI (Tauri)** | **COMPLETE** | Written from source + spot-checked. 4 files. |
| 20 | **Embedded Database (SQLCipher)** | **COMPLETE** | Written from source + spot-checked. 2 files. |
| 21 | Web Admin Interface | **PHASE 2 DONE** | All 40+ code listings match source exactly. 2 files. |

### Part II: Deployment & Operations

| # | Chapter | Status | Notes |
|---|---------|--------|-------|
| 22 | Docker Compose | **PHASE 2 DONE** | 1 fix: Dockerfile updated (Rust version, Node.js stage, workspace members). 13 files verified |
| 23 | Kubernetes | **PHASE 2 DONE** | All 20 manifests verified. 8 files. |

### Part III: Language Reference

| # | Chapter | Status | Notes |
|---|---------|--------|-------|
| 24 | Rust Language Guide | **PHASE 2 DONE** | All examples syntactically correct and idiomatic. 18 sub-files verified |

---

## Roadmap (Ordered)

### Phase 1: New Content ✅ COMPLETE
1. ~~**Write Tauri Desktop Admin chapter**~~ → Done: Ch. 4.2 (`bitcoin-desktop-ui-tauri/04.2-*.md`, 4 files)
2. ~~**Write Tauri Wallet chapter**~~ → Done: Ch. 5.2 (`bitcoin-wallet-ui-tauri/05.2-*.md`, 4 files)
3. ~~**Refactor Ch. 6 (Embedded Database)**~~ → Done: standalone `embedded-database/06-*.md` (2 files)
4. ~~**Rename files and update navigation**~~ → Done: All files renamed to 4.1/4.2/5.1/5.2 scheme, all links updated across 120+ files, README.md rewritten

### Phase 2: Technical Accuracy Pass ✅ COMPLETE
4. ~~**Verify every code example**~~ → Done. All chapters verified against source. Fixes applied:
   - **Ch.2 Primitives**: Fixed `Block::new_block()` parameter order, `Transaction::new_utxo_transaction()` signature (4 params not 5), replaced non-existent `Blockchain::new()` with `BlockchainService::initialize()`
   - **Ch.3 Web API**: Fixed `create_app()` return type to `Result<Router, ...>` in 5 files (02-Server-Setup, 09-OpenAPI, Axum.md, Tower.md), fixed `BalanceResponse` struct fields in 06-Data-Models
   - **Ch.4.1 Iced Admin**: Fixed comments in runtime.rs code listing (04.1A) to match source verbatim
   - **Ch.8 Docker**: Updated Dockerfile listing from `rust:1.70-slim` to `rust:1.91.1-slim`, added Node.js build stage, added workspace members
   - **Ch.4.2, 5.1, 5.2, 6, 7, 9, 10**: All verified — no fixes needed
   - **Ch.1.3-1.4 Whitepaper**: Conceptual check passed — all Rust code syntactically valid and internally consistent
   - **Ch.10 Rust Guide**: All pedagogical examples verified — syntactically correct and idiomatic

### Phase 3: Editorial Quality Pass ✅ COMPLETE
5. ~~**Writing quality sweep**~~ → Done. All main chapter files reviewed. Key fixes:
   - **"We" language**: Applied consistently across Ch.1.2, 2.1–2.9, 3 (6 files), 5.1, 5.2, 6, 7, 8, 9, 10
   - **Typos**: "implementaion" → "implementation" (Ch.4.1), "trasaction" → "transaction" (Ch.2.4)
   - **Duplicate sections removed**: Ch.1 (closing paragraph), Ch.3/01-Introduction (repeated paragraph), Ch.8 (duplicate Wallet Address Distribution section)
   - **Tone/voice**: Converted passive → active throughout; improved pedagogical flow
   - **Formatting**: Normalized em-dashes, fixed section header punctuation, added missing prose before code blocks
   - **Section numbering**: Fixed Ch.10/01-Introduction heading sequence (5.5 → 6, then cascaded)

### Phase 4: Final Polish ✅ COMPLETE
6. ~~**Update README.md**~~ → Done (included in Phase 1 restructuring)
7. ~~**Update all navigation elements**~~ → Done (included in Phase 1 restructuring)
8. ~~**Cross-reference audit**~~: 6,135 links verified across 127 files — all valid. Fixed 20 breadcrumb navigation errors (wrong chapter numbers, wrong backward links in web sub-chapters, "Section" vs "Chapter" inconsistency)
9. ~~**Final read-through**~~: Coherence check across all chapters complete. Fixed incorrect prev/next chapter references in Tauri chapters, Docker Compose sub-chapters, and web API sub-chapters

---

## Tauri Chapter Architecture Notes

### Desktop Admin (Tauri) — Key Topics to Cover

**Rust Backend** (`src-tauri/`):
- `main.rs`: Tauri app setup, command registration, state management
- `commands/`: 6 modules (blockchain, wallet, transactions, mining, health, settings)
- `services/bitcoin_api.rs`: BitcoinApiService wrapping AdminClient
- `models/mod.rs`: Data models (WalletAddress, BlockSummary, etc.)
- `config/mod.rs`: ApiConfig with RwLock for thread-safe mutation
- Pattern: All commands are `#[tauri::command]` async functions returning `Result<Value, String>`

**React Frontend** (`src/`):
- 18 pages across 5 sections (Blockchain, Wallet, Transactions, Mining, Health)
- Key components: AppLayout, Sidebar, TopBar, StatusBar, DataCard, DataTable
- State: Zustand (`useAppStore`) + React Query for data fetching
- Forms: react-hook-form + zod validation
- Styling: Tailwind CSS with dark/light theme
- IPC: `invoke()` from `@tauri-apps/api/core`

**Key Architectural Contrast with Iced**:
- Iced: Pure Rust, MVU (Model-View-Update), single-language
- Tauri: Rust backend + React frontend, IPC bridge, two-language
- Both use the same `bitcoin-api` crate for API communication
- Both offer identical blockchain management features

### Wallet (Tauri) — Additional Topics

- SQLCipher encrypted database (same pattern as Iced wallet)
- `database/mod.rs`: Schema v2, migrations, deterministic password generation
- Tables: settings, wallet_addresses, users
- Wallet CRUD with React Query cache invalidation
- Test suite (`src/test/`) with mocks for Tauri APIs

---

## Known Issues & Watch Items

- [x] Chapter numbering is inconsistent (1, 1.2, 1.3, 1.4, 2.0, 2.1... 3, 4.1, 4.2, 5.1, 5.2...) — **FIXED**: Flattened to Chapters 1–24 across four Parts. All 124 files renumbered, navigation blocks updated, breadcrumbs updated, prose cross-references updated, Part labels updated.
- [x] Some chapters reference "Chapter 2.6" for both Block Acceptance AND Blockchain State Management — **FIXED**: Block Acceptance is now Ch 10, Storage Layer is Ch 11, all references corrected
- [x] README badge says "10 Chapters" — **FIXED**: Updated to 24
- [x] Ch.6 is Iced-specific but database patterns are shared with Tauri — **FIXED**: refactored into standalone `embedded-database/` chapter (now Ch 20)
- [x] Tauri apps not mentioned anywhere in current book — **FIXED**: Ch 17 (Admin) and Ch 19 (Wallet) written
- [x] Need to decide chapter numbering for new Tauri chapters — **FIXED**: Ch 16/17 (Desktop Admin) and Ch 18/19 (Wallet)
- [x] Old Iced-specific DB files still exist in `bitcoin-wallet-ui-iced/` — **FIXED**: Deleted, superseded by standalone `embedded-database/`
- [x] Web UI chapter (Ch 21) may need updating if the Tauri web frontend shares patterns — **RESOLVED**: Detailed analysis confirmed Ch 17, 19, and 21 share React Query patterns but solve fundamentally different architectural problems (Tauri IPC stateless, Tauri IPC + SQLCipher, HTTP REST). Added cross-references connecting the three chapters. No content deduplication needed.
- [x] Verify all `bitcoin-api` crate references are consistent across chapters — **VERIFIED**: 30+ references across main chapters all consistent. Glossary defines it, Appendix shows it in directory trees, Ch 15–19 reference it correctly as the shared workspace crate.

---

## Open Questions

> Questions that need user input before proceeding. Do not resolve these by guessing — ask the user.

- [x] **Chapter numbering**: ~~Should we renumber?~~ **RESOLVED**: Flattened to Chapters 1–24 across four Parts: Part I Foundations & Core Implementation (Ch 1–21), Part II Deployment (Ch 22–23), Part III Reference (Ch 24). All 124 files renumbered.
- [x] **Old Iced-specific database chapter**: ~~Should we delete?~~ **RESOLVED**: Deleted both old files. Standalone `embedded-database/06-*` is the single source of truth.
- [x] **Phase 2 scope**: ~~Which chapters?~~ **RESOLVED**: All chapters (1-10), including blockchain core, all UIs, deployment, and Rust guide.
- [x] **Phase 2 fix approach**: ~~Fix silently, log, or ask?~~ **RESOLVED**: Fix silently — automatically correct mismatches to match the actual codebase.
- [x] **Rust Language Guide (Ch. 10)**: **RESOLVED**: Included in Phase 2 scope (all chapters).
- [x] **Deployment chapters (Docker/K8s)**: **RESOLVED**: Included in Phase 2 scope (all chapters).
- [x] **Navigation update scope**: ~~Should we update ALL chapter navigation sidebars?~~ **RESOLVED**: All 120+ files updated with new chapter numbering and links. Tauri chapters (4.2, 5.2) and standalone database chapter (6) are now linked from all navigation sidebars.
- [x] **ACTION REQUIRED — Repository URL and QR code**: **RESOLVED**: Repository URL set to https://github.com/bkunyiha/rust-blockchain. Updated in 00-Quick-Start.md (clone command) and Appendix-Source-Reference.md (source reference link). QR code can be generated for the print edition during LaTeX conversion.

---

## Session Log

| Date | Work Done |
|------|-----------|
| 2026-03-15 | Initial context file created. Explored full project structure, both Tauri codebases, and existing Iced chapter style. Defined roadmap. |
| 2026-03-15 | Completed Tauri Desktop Admin chapter (4 files): 08-Tauri-Desktop-Admin-UI.md, 08A-Tauri-Rust-Backend.md, 08B-Tauri-Frontend-Infrastructure.md, 08C-Tauri-Frontend-Pages.md. All verbatim code + annotations. |
| 2026-03-15 | Starting Tauri Wallet UI chapter (bitcoin-wallet-ui-tauri). |
| 2026-03-15 | Completed Tauri Wallet UI chapter (4 files): 09-Tauri-Wallet-UI.md, 09A-Tauri-Wallet-Rust-Backend.md, 09B-Tauri-Wallet-Frontend-Infrastructure.md, 09C-Tauri-Wallet-Frontend-Pages.md. All verbatim code + annotations. |
| 2026-03-15 | Refactored Ch.6 Embedded Database into standalone chapter in `embedded-database/`. Now framework-agnostic, covering both Iced and Tauri wallet SQLCipher implementations. 2 files: 06-Embedded-Database.md (annotated walkthrough) + 06A-Embedded-Database-Code-Listings.md (verbatim source from both wallets + tests + password generation). Old Iced-specific versions remain in `bitcoin-wallet-ui-iced/` for now. |
| 2026-03-15 | Restructured chapter numbering: Ch.4 → 4.1 (Iced Admin) + 4.2 (Tauri Admin), Ch.5 → 5.1 (Iced Wallet) + 5.2 (Tauri Wallet). Renamed all 16 chapter files. Updated all inter-chapter links across 120+ files. Rewrote README.md with new structure, companion chapter listings, and Tauri content throughout. Updated BOOK-CONTEXT.md chapter status tracker, roadmap, known issues, and open questions. |
| 2026-03-16 | Updated top chapter navigation in all 127 book files with complete chapter list including all companions. Deleted old superseded database files. |
| 2026-03-16 | **Phase 2 Technical Accuracy Pass COMPLETE.** Verified all chapters against source code. Fixes: Ch.2.1 (3 function signature fixes), Ch.3 (5 files: create_app() return type, BalanceResponse fields), Ch.4.1A (runtime.rs comments), Ch.8 (Dockerfile updated: Rust 1.91.1, Node.js stage, workspace members). Ch.7 web UI, Ch.9 K8s, Ch.1.3-1.4 whitepaper, Ch.10 Rust guide — all verified clean. Ch.4.2, 5.2, 6 spot-checked clean. |
| 2026-03-16 | **Phase 3 Editorial Quality Pass COMPLETE.** Reviewed all main chapter files. Applied "we" language consistency, fixed typos (2), removed duplicate sections (3), improved passive→active voice, normalized formatting (em-dashes, section headers, code block introductions), fixed heading numbering in Ch.10. |
| 2026-03-16 | **Phase 4 Final Polish COMPLETE.** Cross-reference audit: verified all 6,135 links across 127 files — zero broken links. Coherence read-through: fixed 20 breadcrumb navigation errors (wrong chapter numbers in Ch.4.2, 4.2C, 5.2C, Docker Compose sub-chapters; wrong backward links in 11 web API sub-chapters; "Section" → "Chapter" terminology in Ch.2.2/2.3). All four roadmap phases now complete. |
| 2026-03-16 | **Print remediation COMPLETE.** Code blocks: all 1,274 main-chapter blocks now ≤80 chars wide, ≤60 lines long, and language-tagged (120 untagged blocks fixed — 117 `text`, 3 corrected misclassifications). Companion chapters (15 files, 17,315 lines): excluded from print, replaced by `Appendix-Source-Reference.md` with annotated directory trees per module, repository link placeholder, and summary table. |
| 2026-03-16 | **Content trimming: ~96 pages cut.** Rust Language Guide (19% reduction), Docker Compose consolidated from 12→6 chapters (old files deleted), web framework guides merged (Tower→Axum, Serde+Utoipa), Tauri Admin/Wallet restructured with cross-references to Iced chapters, whitepaper business objects deduplicated against Domain Model, Std-vs-Tokio and Transaction ID Format trimmed. Total: ~830→~734 estimated pages. |
| 2026-03-16 | **Book reorganization.** (1) Swapped Ch 2.5 and 2.6: Block Acceptance now Ch 2.5 (follows Blockchain Core directly), Storage Layer now Ch 2.6. Updated 250+ references across 120 files. Fixed navigation block line ordering so 2.5 appears before 2.6. (2) Added async Rust prerequisite note to Ch 3 (Web API) pointing readers to Tokio Runtime Guide and Ch 10 Rust Language Guide. (3) Added bridge paragraph at start of Ch 2.0 (Rust-Project-Index) mapping the whitepaper's 5-step roadmap (Bytes→Identity→Authorization→State→Consensus) to specific chapter numbers 2.1–2.9. |
| 2026-03-16 | **Readability review and fixes.** (1) Expanded Ch 1 introduction — added book motivation, target audience, and "How the book is organized" section explaining Part I/II/III arc; replaced scattered Navigation links with focused "What to read next". (2) Replaced Ch 2.1 Primitives skeleton "Topics to Cover" outline with real prose: Design Decisions (pure data, Vec<u8>, Serde) and How Primitives Connect to Later Chapters; fixed closing paragraph from "Storage Layer" to "Utilities". (3) Fixed Ch 1.3 header from "Part II: Chapter 2.3" to "Part I: Chapter 1.3". (4) Added brevity note to Ch 2.2 Utilities. (5) Made Ch 10 Rust Guide placement clearer in Ch 1 and Ch 2.0 ("read it before, not after"). (6) Fixed stale breadcrumbs in chain/README.md (top pointed to Storage instead of Block Acceptance, "after Ch 2.4" list had 2.6 before 2.5). (7) Fixed net/README.md back-link from Ch 2.5 to Ch 2.6. (8) Fixed README.md detailed descriptions section — renumbered all 9 chapter descriptions to match canonical table. (9) Deleted stale chain/08-Node-Orchestration.md.bak. |
| 2026-03-16 | **Pedagogical scaffolding pass (session 2).** Added Prerequisites + "What You'll Learn" callouts to 10 chapters: Store (2.6), Node (2.8), Wallet (2.9), Network (2.7), Iced (4.1), Tauri (4.2), Embedded DB (6), Web Admin (7), Docker (8), K8s (9), Rust Guide (10). Added 4 framework introductions: sled (2.6), Iced (4.1), Tauri (4.2), SQLCipher (6). Added "Why This Matters for Bitcoin" context to Storage (2.6), Network (2.7), Docker (8), K8s (9). |
| 2026-03-16 | **Publication-readiness pass.** (1) Created 00-Quick-Start.md — a 2-page "see it run" experience: clone, docker-compose up, see blocks mined. (2) Added system architecture diagram (ASCII art) to Ch 1 showing full crate dependency flow. (3) Added "What This Book Does Not Cover" to Ch 1 with 6 explicit scope exclusions. (4) Created Glossary.md with 40+ terms across 3 sections (Bitcoin, Rust, project-specific). (5) Added checkpoint moments to 5 milestone chapters (2.5 Block Acceptance, 2.6 Storage, 2.8 Node, 3 Web API, 8 Docker). (6) Added Common Errors troubleshooting sidebars to Crypto (2.3), Storage (2.6), Network (2.7). (7) Added Further Reading sections to Crypto (2.3), Storage (2.6), Network (2.7), Wallet (2.9), Web API (3), Docker (8). (8) Fixed voice consistency — standardized Ch 2.1 Primitives from 3rd-person to 1st-person-plural. (9) Updated README.md nav to include Ch 0 Quick Start, Glossary, and Appendix links. (10) Created Editorial-Recommendations.docx documenting all 12 recommendations with status tracking. |
| 2026-03-16 | **Tone/language rewrite — blog-to-book transformation.** Full pass across all 120+ files converting blog-style writing into professional technical book prose. Changes: (1) Removed all emoji from all markdown files globally (📚, 📘, 📑, 📄, 🗺️, 🚀, 🧹, 📖, 💡) — preserved only emoji in actual code listings (📋 in UI button text). (2) Removed contract paragraphs ("The goal is that you can read this chapter without the repository open...") from 7+ files (Ch 2.6, 2.7, 2.8, 2.9, 4.1, 4.2, and others). (3) Converted all "Methods involved" blockquote boxes into inline prose sentences or removed them entirely — 43+ instances across all chapter files. (4) Removed closing meta-commentary paragraphs ("*This chapter has introduced...*") from all chapters. (5) Removed "In one sentence" hooks from UI chapters. (6) Removed "Our Learning Journey" syllabus previews from Docker and K8s chapters. (7) Removed "Quick Start" / "Learning Paths" blog-style navigation sections from Ch 3. (8) Removed enthusiasm markers ("Ready to begin?", "Let's get started!", exclamation marks). (9) Removed self-referential meta-commentary about the book format. (10) Converted bloated bullet-list introductions into concise prose paragraphs (Ch 1.2, Ch 2.3, Ch 3). (11) Cleaned 📘 prefix from all Prerequisites lines. (12) Removed redundant "Related Implementation Chapters" bullet-list sections, replaced with prose. |
| 2026-03-18 | **Pre-print editorial recommendations implemented (15 of 15).** Learning objectives added to all 24 chapters. Chapter summaries added to 18 chapters. Exercises added to Ch 6–23 (38 total). 6 formal ASCII diagrams added (Ch 6, 9, 10, 15, 20, 22). Callout boxes (tips, warnings, notes) added to all 24 chapters. Further reading sections added to 16 chapters missing them. Transition bridges connecting all 20 sequential chapter pairs. Prerequisites matrix added to front matter. Companion chapter guidance added to all 8 chapters with companions. Scope disclaimers added to 10 chapters. Code width violations fixed (zero remaining in main chapters). Front matter completed: Dedication, Acknowledgments, author name (Bill Kunyiha). Bibliography.md created with 20+ references. Glossary expanded with 16 new terms. |
| 2026-03-18 | **Professional front matter restructured.** README.md reorganized into standard print order: Half title → Title page → Copyright → Dedication → Acknowledgments → Table of Contents → Preface (with Who This Book Is For, Technologies You Will Learn, Book Structure, How to Read) → Chapter Prerequisites → Suggested Reading Paths → Cleanup → Resources. |
| 2026-03-18 | **Technology differentiation.** Added "Technologies You Will Learn" table to Preface (12 technologies mapped to chapters). Added "Framework Comparison" callouts to Ch 16–19 (Iced vs Tauri side-by-side). Updated "Who This Book Is For" to explicitly list Axum, Tokio, Iced, Tauri 2, Sled, SQLCipher, serde, secp256k1. |
| 2026-03-18 | **Title evolution.** (1) "Building a Full-Stack Bitcoin Blockchain With Rust" → "Full-Stack Rust: Building a Bitcoin Blockchain From Whitepaper to Production" → "Full-Stack Rust: Building a Blockchain From Whitepaper to Production" → final: "Rust Blockchain: A Full-Stack Implementation Guide." (2) Chapter 2 renamed from "Introduction to Bitcoin & Blockchain" to "Introduction to Blockchain" across all 126 files. (3) Subtle Bitcoin references added throughout (e.g., "modeled on Bitcoin's architecture," "like the one Bitcoin runs on," "following the Bitcoin model"). |
| 2026-03-18 | **Ch 17/19/21 cross-reference analysis.** Confirmed shared React Query patterns across Tauri and Web chapters; added cross-references to Ch 17 (React Query note) and Ch 21 (architecture comparison sidebar). Marked BOOK-CONTEXT item as resolved. |
| 2026-03-18 | **Grammar and cleanup passes.** Fixed "trasaction" typo in Ch 9.2/9.3. Fixed duplicate "## Chapter 1" heading. Fixed `align="middle"` → `align="center"`. Removed 11 duplicate "What you will learn" lines. Removed 6 double `---` separators. Fixed duplicate "## Summary" in Rust Ch 14. Fixed stale chapter numbers in Appendix summary table (6/7/8/9 → 20/21/22/23). Fixed Glossary chapter reference ordering. Fixed "Bitcoin & Blockchain" in prerequisites table. Fixed Quick Start navigation (added missing Ch 18–19). Fixed Ch 4 "Part III" → "Part I" label. Renamed Ch 24 internal "Part I–VII" → "Section I–VII" to avoid collision with book Parts. |
| 2026-03-18 | **Verified bitcoin-api crate references.** All 30+ references consistent across main chapters. Marked BOOK-CONTEXT item resolved. All BOOK-CONTEXT known issues now resolved except repository URL + QR code (requires author input). |
| 2026-03-18 | **Supporting documents created.** (1) Cover-Image-Recommendations.docx — Midjourney + Canva workflow, color palette, typography, AI prompts, print specs. (2) Book-Marketing-Syndication-Plan.docx — multi-platform sales (KDP + Gumroad + Leanpub), blog syndication (Hashnode/Dev.to/Medium), This Week in Rust strategy, ARC outreach, SEO keywords, 8-week content calendar. (3) Dedication and Acknowledgments finalized (Option C dedication, Option B acknowledgments). |
| 2026-03-16 | **Chapter renumbering — flat numbering scheme.** Flattened the inconsistent numbering (1, 1.2, 1.3, 1.4, 2.0, 2.1–2.9, 3, 4.1, 4.2, 5.1, 5.2, 6, 7, 8, 9, 10) to Chapters 1–24 across four Parts. Part I: Foundations & Core Implementation (Ch 1–21), Part II: Deployment (Ch 22–23), Part III: Reference (Ch 24). Mapping: old 1.2→2, 1.3→3, 1.4→4, 2.0→5, 2.1→6, …, 2.9→14, 3→15, 4.1→16, 4.2→17, 5.1→18, 5.2→19, 6→20, 7→21, 8→22, 9→23, 10→24. Companion chapters renumbered (4.1A→16A, etc.). Sub-chapters 2.4.1–2.4.9→9.1–9.9. All 124 markdown files updated via Python script with placeholder-token approach to prevent cascading. Manual fixes: umbrella "Chapters 4–5" refs, README section headers, badge anchors, architecture diagram alignment, one corrupted whitepaper section range (Sections 7–8). Updated BOOK-CONTEXT.md status tracker, known issues, and open questions. |
