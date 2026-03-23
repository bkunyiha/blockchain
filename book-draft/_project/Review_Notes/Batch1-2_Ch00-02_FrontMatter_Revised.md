# Batch 1–2: Front Matter & Chapter 2 — Revised Sections

Only changed sections are shown below. For each change, the file path, original text, and revised text are provided.

---

## Change 1: Standardize `docker compose` syntax (00-Quick-Start.md)

**File:** `00-Quick-Start.md`
**Line:** 71

**Original:**
```
docker-compose up --build
```

**Revised:**
```
docker compose up --build
```

**Also line 135:**

**Original:**
```
docker-compose down -v    # Stop containers and remove volumes
```

**Revised:**
```
docker compose down -v    # Stop containers and remove volumes
```

**Reason:** Docker Compose v2 uses space-separated `docker compose`. The README.md Cleanup section already uses this form. Standardize throughout.

---

## Change 2: Build time estimate (00-Quick-Start.md)

**File:** `00-Quick-Start.md`
**Line:** 73

**Original:**
```
This builds the Rust node from source (first run takes a few minutes while Cargo compiles), then starts two containers: a **miner** node and a **webserver** node.
```

**Revised:**
```
This builds the Rust node from source (the first run takes several minutes while Cargo compiles all dependencies), then starts two containers: a **miner** node and a **webserver** node.
```

**Reason:** "A few minutes" understates the time on slower hardware. "Several minutes" sets better expectations without over-specifying.

---

## Change 3: Remove duplicate breadcrumb (01-Introduction.md)

**File:** `01-Introduction.md`
**Lines:** 57–64

**Original:**
```markdown
---
<div align="center">

**[← Back to Main Book](README.md)** | **[Next: Introduction to Blockchain →](bitcoin-blockchain/README.md)**

</div>

---
```

**Revised:** *(Delete these 8 lines entirely.)*

**Reason:** The breadcrumb at lines 66–72 already provides full navigation. Having two consecutive breadcrumb blocks is redundant and not part of the standard chapter template.

---

## Change 4: Rename closing section (01-Introduction.md)

**File:** `01-Introduction.md`
**Line:** 234

**Original:**
```markdown
## What We Covered
```

**Revised:**
```markdown
## Summary
```

**Reason:** "What We Covered" is a blog-style heading pattern. "Summary" is the standard print convention and matches the style guide's prohibition on self-referential meta-commentary.

---

## Change 5: Fix subtitle (README.md)

**File:** `README.md`
**Line:** 95

**Original:**
```markdown
### A Full-Stack Implementation Guide With Tokio/Async Rust, Axum, Iced, Tauri 2, Docker, and Kubernetes
```

**Revised:**
```markdown
### A Full-Stack Implementation Guide Featuring Tokio, Axum, Iced, Tauri 2, Docker, and Kubernetes
```

**Reason:** The slash in "Tokio/Async Rust" is informal for a print subtitle. "Featuring" is standard book-subtitle phrasing. Tokio implies async Rust; no need to state both.

---

## Change 6: Fix contraction (README.md)

**File:** `README.md`
**Line:** 375

**Original:**
```markdown
When you're done experimenting, you can tear down resources cleanly.
```

**Revised:**
```markdown
When you are done experimenting, you can tear down resources cleanly.
```

**Reason:** Formal tone consistency. The book avoids contractions elsewhere.

---

## Change 7: Remove "continuously updated" meta-commentary (README.md)

**File:** `README.md`
**Line:** 445

**Original:**
```markdown
*This documentation is continuously updated as the book writing process progresses. For the most current information, please refer to the latest version of each chapter.*
```

**Revised:** *(Delete this line entirely.)*

**Reason:** This signals an unfinished draft, inappropriate for a print edition. The digital/markdown version can retain it as an HTML comment if needed.

---

## Change 8: Tighten "Who This Book Is For" bullet (README.md)

**File:** `README.md`
**Line:** 237

**Original:**
```markdown
- **Technologists learning Rust and its ecosystem** who want hands-on experience with Axum, Tokio, Iced, Tauri 2, Sled, SQLCipher, serde, and secp256k1 — all through one cohesive codebase rather than isolated tutorials
```

**Revised:**
```markdown
- **Technologists learning Rust** who want hands-on experience with Tokio, Axum, Iced, Tauri 2, Sled, and SQLCipher through one cohesive codebase rather than isolated tutorials
```

**Reason:** Tightened from 43 words to 27 words. Removed "and its ecosystem" (redundant given the list) and the less-recognizable crate names (serde, secp256k1) that readers at this level already know or will encounter naturally.

---

## Change 9: Move "Note" callout to chapter opening (bitcoin-blockchain/README.md)

**File:** `bitcoin-blockchain/README.md`
**Line:** 201 → move to line 88

**Original (at line 201, mid-section):**
```markdown
> **Note:** This chapter covers Bitcoin as a technology and protocol. We do not cover cryptocurrency trading, investment, or regulatory considerations — our focus is purely on the engineering.
```

**Revised:** Move this callout to immediately after line 87 (after the "What you will learn" box), before the "## What Is Bitcoin?" heading. Delete it from its current location at line 201.

**Reason:** Scope disclaimers belong at the chapter opening, not buried mid-section where the reader has already invested 100+ lines of reading.

---

## Summary Table

| # | File | Change | Impact |
|---|------|--------|--------|
| 1 | 00-Quick-Start.md | `docker-compose` → `docker compose` | Consistency |
| 2 | 00-Quick-Start.md | Build time estimate | Clarity |
| 3 | 01-Introduction.md | Remove duplicate breadcrumb | Structure |
| 4 | 01-Introduction.md | "What We Covered" → "Summary" | Style |
| 5 | README.md | Fix subtitle slash | Professionalism |
| 6 | README.md | "you're" → "you are" | Tone |
| 7 | README.md | Remove meta-commentary | Print-readiness |
| 8 | README.md | Tighten bullet length | Style |
| 9 | Ch 2 README.md | Move "Note" callout | Structure |
