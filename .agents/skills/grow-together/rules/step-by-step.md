# Step-by-Step Development Workflow

Each piece of learning content follows a repeatable 5-step process.

## Step 1: Scope

**Goal:** Define exactly what this document teaches.

- Identify the module and submodule (`docs/learn/{module}/{submodule}/`)
- Write a one-sentence scope statement: "This document teaches X so the reader can Y."
- List prerequisite concepts the reader must already know
- List what is explicitly OUT of scope (to prevent scope creep)

**Output:** A scope paragraph in the task prompt.

## Step 2: Research

**Goal:** Gather authoritative sources before writing a single line.

- Read the relevant section of the Rust Book
- Check docs.rs for the actual API signatures
- Read Rust by Example for code-first perspectives
- Search for RFCs if the feature had one
- Check existing docs/learn/ content for cross-references
- Verify all code examples compile with current stable Rust

**Output:** A list of reference links and key API signatures.

## Step 3: Outline

**Goal:** Structure the document before drafting.

Write an outline with:
- Title (H1)
- Overview paragraph
- Prerequisites (bullet list)
- 3–5 main sections (H2) in logical teaching order
- Key code examples for each section
- Glossarium entries (draft)
- Next Steps (2–3 links)

**Output:** A markdown outline with section headers and placeholders.

## Step 4: Draft

**Goal:** Write the full document following the outline.

- Flesh out each section with explanations and code
- Every claim should be verifiable (link to source or show code)
- Code examples must be correct — run them if possible
- Write in plain English, avoid jargon without definition
- Keep paragraphs short (3–5 sentences max)
- Use lists, tables, and diagrams to break up text

**Output:** A complete document ready for review.

## Step 5: Review & Iterate

**Goal:** Polish and validate the document.

Self-review checklist:
- [ ] Does the title match the scope?
- [ ] Are prerequisites accurate and linked?
- [ ] Do code examples compile? (or marked with `ignore`/`no_run`)
- [ ] Is every term in the Glossarium actually used in the document?
- [ ] Are there at least one internal and one external Next Step link?
- [ ] Does the document satisfy the ecosystem-first rule?
- [ ] Is the tone inclusive and accessible?
- [ ] Does the document follow the content-structure rule?

**Output:** A merged, committed document.

## Iteration

When updating existing content, start from Step 4 (Draft) but check Step 2
(Research) to ensure no information is stale.  Bump the document's last-updated
metadata if applicable.
