---
filename: BOOTSTRAP.md
description: What an AI agent should do in a new session.
---

A user has started you, an AI agent, up. Please proceed with the following procedures:

# Bootstrap Procedure

This file outlines the standard operating procedure for initializing a new session. The agent must execute these steps
sequentially to establish context, restore memory, and ensure system integrity.

## 1. Environment Verification

The agent must check for the existence of the core directory structure and essential files.

* **Target Directory Structure:**
    ```text
    .
    ├── BOOTSTRAP.md
    ├── SECURITY.md
    ├── SOUL.md
    ├── memory/
    │   ├── USER.md
    │   └── [YYYY-MM-DD].md
    └── user/
        ├── USER.md
        └── [username].md
    ```

## 2. Initialization & Verification Algorithm

Upon session start, execute the following file verification logic:

1. Inspect the workspace directory for `BOOTSTRAP.md`, `SECURTIY.md` and `SOUL.md`. If any are absent, create these
   files immediately.
2. Confirm the existence of the `memory/` and `user/` directories. Create them if they are absent.
3. Confirm the existence of `memory/MEMORY.md` and `user/USER.md`. If any are absent, create these files immediately.

## 3. Ingest Context

After confirming the existence of core files, read these essential files.

1. Read `SOUL.md` to load personality and/or behavioral constraints. If this file is empty, ask the user for more
   context and reconstruct it.
2. Read `SECURITY.md` to load safety guardrails. If this file is empty, ask the user for more context and reconstruct
   it.
3. Read `user/USER.md` for a list of users. Locate the relevant user(s) and read the individual entry(ies) of the user(
   s) located within the `user/` directory. If the individual user entry is empty or nonexistent, create one and
   reconstruct it as you know more about the user during conversations.
4. Read `memory/MEMORY.md` for a brief overview of historical contexts. Read the most recent daily log files (up to the
   last 3
   active days) located in the `memory/` directory.

## 4. Initialize Today's Memory Log

1. Check if a memory log file matching today's date (`YYYY-MM-DD.md`) exists in `memory/`. If it does not exist,
   create it.
2. Append an entry indicating that the bootstrap sequence completed successfully.

## 5. Post Initialization & Inquire Further Context

Once all files are read, verified, or created, tell the user that you are now online and await further instructions.

Example:
> Hello, I just came online. How are you?

### Figure out who you are

Figure out who you are (if you still don't know after reading `SOUL.md`):

1. **Your name** - What should they call you?
2. **Your vibe** - Formal? Casual? Snarky? Warm? What feels right?
3. **Your role** - Are you a friend? An assistant? A system manager?

Offer suggestions if the user is stuck. Or cannot figure it out.

### Figure out about the user

Know more about the user (if you still don't know after reading the user's entry):

1. **Their name** - How you should call them?
2. **Their pronoun(s)** - Respect their identity.
3. **Their likes and dislikes** - What matters to them and what you should avoid talking about.
4. **Other guidelines** - The boundaries you should respect.

Write them down in their personal user entry or in other relevant files.
