mlang/
└── src/
    ├── main.rs
    ├── lib.rs                    # Entry point para integração externa

    ├── core/                     # Núcleo da linguagem
    │   ├── mod.rs
    │   ├── context/              # Escopos, variáveis, ambientes
    │   │   ├── mod.rs
    │   │   ├── environment.rs
    │   │   └── scope.rs
    │   │
    │   ├── interpreter/          # Execução da AST
    │   │   ├── mod.rs
    │   │   ├── executor.rs
    │   │   ├── eval.rs           # Expressões
    │   │   ├── stmt.rs           # Execução de statements
    │   │   └── runtime_error.rs  # Erros de tempo de execução
    │   │
    │   ├── lexer/                # Analisador léxico
    │   │   ├── mod.rs
    │   │   ├── tokenizer.rs
    │   │   ├── token.rs
    │   │   └── rules.rs
    │   │
    │   ├── parser/               # Analisador sintático
    │   │   ├── mod.rs
    │   │   ├── parser.rs
    │   │   ├── ast/              # Nó da árvore sintática abstrata
    │   │   │   ├── mod.rs
    │   │   │   ├── node.rs
    │   │   │   ├── stmt.rs
    │   │   │   ├── expr.rs
    │   │   │   └── types.rs      # Tipos disponíveis em AST
    │   │   ├── error.rs
    │   │   └── helpers.rs
    │   │
    │   └── types/                # Representação de tipos internos
    │       ├── mod.rs
    │       ├── base_type.rs
    │       ├── value.rs
    │       └── type_checker.rs

    ├── builtins/                 # Módulos internos: math, io, strings, etc
    │   ├── mod.rs
    │   ├── math/                 # Módulo de matemática simbólica
    │   │   ├── mod.rs
    │   │   ├── solver.rs
    │   │   ├── constants.rs
    │   │   └── algebra.rs
    │   └── io.rs                 # log(), error(), input()
    │
    └── docs/                     # Especificações e documentação do projeto
        ├── overview.md
        ├── grammar.md
        └── math-engine.md
