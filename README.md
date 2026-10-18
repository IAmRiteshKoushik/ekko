# Ekko: A Workflow Engine

Suggest file structure by `GPT-6 Luna`. Will see whether it is worth 
implementing this or whether I want to go in a different direction.

```
durable-engine/
├── Cargo.toml
└── src/
    ├── lib.rs                 # shared types and application logic
    ├── bin/
    │   ├── server.rs          # API and scheduler startup
    │   ├── worker.rs          # worker startup and handler registration
    │   └── engine.rs          # CLI commands
    ├── domain/
    │   ├── mod.rs
    │   ├── workflow.rs
    │   └── task.rs
    ├── store/
    │   ├── mod.rs
    │   └── postgres.rs
    ├── server/
    │   ├── mod.rs
    │   ├── api.rs
    │   └── scheduler.rs
    └── worker/
        ├── mod.rs
        └── runner.rs
```

