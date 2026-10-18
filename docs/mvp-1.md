# Out of scope

- Workflow code replay and deterministic execution
- Timers, signals, child workflows, and workflow versioning
- Exactly-once side effects
- Advanced scheduling, and autoscaling
- A general workflow DSL or client SDK
- Rich UI, search and metrics infrastructure

# In scope

1. Workflow and task model
  - A workflow is a named run with a unique ID
  - It is made of a fixed sequence of tasks
  - Each task has a stable ID, type, input, status, attempt count and result or error
2. Durable job server
  - Persist workflow and task state in a DB
  - Provide APIs to start workflow, inspect state, cancel it
  - Track states such as `pending`, `running`, `completed`, `cancelled`
  - Record each state transition so the server can recover and explain what 
    happened
3. Worker execution
  - Workers poll or long-poll for eligible tasks, claim one, execute it and 
  report success or failure.
  - Use a lease of visibility timeout for claims. If worker disappears, the 
  task becomes eligible after the lease expires
  - Support a configured retry policy: maximum attempts and a simple backoff
4. Crash recovery
  - On server restart, find expired claims and make those tasks eligible again
  - On worker restart, allow unfinished work to be reclaimed
  - Persist results before marking tasks complete, and make completion reports 
  idempotent so retries do not corrupt state
5. Basic ops
  - Structured logs with workflow ID, task ID, worker ID, and attempt
  - CLI-trigger for starting and inspecting workflows
