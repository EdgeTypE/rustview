# Architecture

This document describes how RustView works internally.

---

## High-Level Overview

```
┌─────────────────────────────────────────────┐
│                  Browser                     │
│  ┌─────────────────────────────────────┐    │
│  │          DOM (patched by shim)       │    │
│  │  ← SSE patches ← Server diffs      │    │
│  │  → HTTP POST → widget events        │    │
│  └─────────────────────────────────────┘    │
└─────────────────────────────────────────────┘
                     ↕ HTTP / SSE
┌─────────────────────────────────────────────┐
│               Axum Server                    │
│  ┌──────────┐  ┌────────┐  ┌────────────┐  │
│  │SessionStore│ │ Router │  │ SSE Stream │  │
│  │ (DashMap)  │ │        │  │            │  │
│  └──────────┘  └────────┘  └────────────┘  │
│         ↕                                    │
│  ┌──────────────────────────────────────┐   │
│  │         App Function Re-run          │   │
│  │  fn app(ui: &mut Ui) { ... }         │   │
│  │         ↓ builds VNode tree          │   │
│  └──────────────────────────────────────┘   │
│         ↕                                    │
│  ┌──────────────────────────────────────┐   │
│  │    VNode Diff (old tree vs new tree)  │   │
│  │    Produces: [Patch::Replace, ...]    │   │
│  └──────────────────────────────────────┘   │
└─────────────────────────────────────────────┘
```

---

## Core Components

### 1. VNode — Virtual DOM

RustView maintains a virtual DOM tree (`VNode`). Each widget call appends one or more `VNode`s to the tree.

```rust
pub struct VNode {
    pub id: String,          // Unique node ID
    pub tag: String,         // HTML tag (div, input, button, etc.)
    pub attrs: HashMap<String, String>,  // HTML attributes
    pub children: Vec<VNode>,  // Child nodes
    pub text: Option<String>,  // Text content
}
```

Key operations:
- `VNode::new(id, tag)` — create a node
- `.with_attr(key, value)` — add an attribute
- `.with_child(child)` — add a child node
- `.with_text(text)` — set text content

### 2. Diffing

After each app re-run, RustView diffs the new VNode tree against the previous one stored in the session. The diff produces a list of patches:

- **Replace** — swap out a node entirely
- **UpdateAttrs** — change attributes on an existing node
- **UpdateText** — change text content
- **AppendChild** / **RemoveChild** — add or remove children

Only changed nodes are sent to the browser, minimizing data transfer.

### 3. Session Management

Each browser tab gets a unique session:

```
SessionStore (DashMap<Uuid, Session>)
  └── Session
       ├── id: Uuid
       ├── widget_state: HashMap<String, Value>   // Widget current values
       ├── user_state: HashMap<String, Value>      // User-defined state
       ├── last_tree: Option<VNode>                // Previous render tree
       └── last_active: Instant                    // For TTL expiry
```

Sessions are stored in a concurrent `DashMap` and automatically cleaned up when they exceed the TTL.

### 4. Server (Axum)

RustView runs an Axum HTTP server with these routes:

| Route | Method | Purpose |
|-------|--------|---------|
| `/` | GET | Serve the HTML page with embedded CSS and JS shim |
| `/api/session` | POST | Create a new session, return UUID |
| `/api/event` | POST | Receive widget events (clicks, input changes) |
| `/api/sse/:id` | GET | SSE stream for pushing DOM patches to the client |
| `/api/upload` | POST | Receive file uploads |

### 5. Client Shim

The HTML page includes a minimal JavaScript shim (~150 lines) that:
1. Connects to the SSE endpoint
2. Receives DOM patch instructions as JSON
3. Applies patches to the live DOM
4. Sends user events back via POST

No npm, no bundler, no React — just a small inline script.

---

## Request Flow

1. **Page Load**: Browser requests `/` → receives HTML with CSS + JS shim
2. **Session Init**: Shim POSTs to `/api/session` → receives session UUID
3. **SSE Connect**: Shim opens SSE connection to `/api/sse/{uuid}`
4. **Initial Render**: Server runs `app(ui)` → builds VNode tree → diffs against empty tree → sends full tree as patches via SSE
5. **User Interaction**: User types in a text input → shim POSTs `{widget_id, value}` to `/api/event`
6. **Re-render**: Server updates session state → re-runs `app(ui)` → diffs new tree vs old tree → sends only changed patches via SSE
7. **DOM Patch**: Shim receives patches → updates DOM in-place

---

## Widget ID Generation

Each widget gets a unique ID based on render order:

- **Auto IDs**: `w-0`, `w-1`, `w-2`, ... (incremented per render)
- **Keyed IDs**: `k-username`, `k-email` (set via `ui.with_key("...")`)

The ID is used to:
- Match widget values in session state
- Target specific DOM nodes for patching
- Associate events with their source widget

---

## Crate Structure

```
rustview/
├── src/
│   ├── lib.rs          # Crate root, re-exports, run() functions
│   ├── ui.rs           # Ui struct — the main API surface
│   ├── widgets/
│   │   └── mod.rs      # Widget render functions (VNode builders)
│   ├── vdom/
│   │   └── mod.rs      # VNode struct, diffing algorithm
│   ├── server/
│   │   └── mod.rs      # Axum server, routes, SSE, HTML/CSS
│   ├── session/
│   │   └── mod.rs      # Session, SessionStore
│   ├── cache/
│   │   └── mod.rs      # @cached decorator support
│   ├── interface.rs    # Interface (Gradio-style) API
│   └── testing/
│       └── mod.rs      # TestUi test harness
├── rustview-macros/     # Proc macros (#[app], #[cached])
├── examples/           # Example apps
├── docs/               # This documentation
└── KARAR.MD            # Decision log (Turkish)
```

---

## Design Decisions

All architectural decisions are logged in `KARAR.MD` at the project root. Key decisions include:

- **Axum over Actix**: Chosen for async-first design and Tower ecosystem
- **SSE over WebSocket**: Simpler, unidirectional, auto-reconnect built-in
- **VNode diffing**: Minimal data transfer, no full page reloads
- **DashMap for sessions**: Lock-free concurrent access
- **Inline SVG charts**: Zero JavaScript chart dependencies
- **No Polars dependency for core dataframe**: Lightweight built-in DataFrame struct
