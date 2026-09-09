                    ┌────────────────────┐
                    │       Source       │
                    └─────────┬──────────┘
                              │
                              ▼
                    ┌────────────────────┐
                    │       Lexer        │
                    └─────────┬──────────┘
                              │
                    ┌─────────┴──────────┐
                    │                    │
                 Tokens             Diagnostics
                    │
                    ▼
             ┌───────────────┐
             │    Parser     │
             │               │
             │ • VSX         │
             │ • TS syntax   │
             │ • >> splitting│
             │ • ASI         │
             └───────┬───────┘
                     │
                     ▼
             ┌───────────────┐
             │      AST      │
             └───────┬───────┘
                     │
                     ▼
        ┌─────────────────────────┐
        │    Semantic Analysis    │
        │                         │
        │ • scopes                │
        │ • bindings              │
        │ • null → undefined      │
        │ • let relationships     │
        │ • rel relationships     │
        │ • function analysis     │
        │ • effect classification │
        │ • exports               │
        └────────────┬────────────┘
                     │
                     ▼
             ┌───────────────┐
             │  Lowering/IR  │  ← optional initially
             └───────┬───────┘
                     │
                     ▼
             ┌───────────────┐
             │   Interpreter │
             │      / VM     │
             └───────┬───────┘
                     │
                     ▼
             ┌───────────────┐
             │ Reactive      │
             │ Runtime       │
             │               │
             │ • bindings    │
             │ • dependency  │
             │   graph       │
             │ • effects     │
             │ • scheduler   │
             └───────────────┘