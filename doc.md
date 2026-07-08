                    cargo run
                        │
                        ▼
                  src/main.rs
                        │
                        ▼
            collector::collect_identity()
                        │
        ┌───────────────┴────────────────┐
        ▼                                ▼
hostname::get_hostname()        machine_id::get_machine_id()
        │                                │
        ▼                                ▼
 System::host_name()           Read /etc/machine-id
        │                                │
        └───────────────┬────────────────┘
                        ▼
                 NodeIdentity Struct
                        │
                        ▼
      serde_json::to_string_pretty(&identity)
                        │
                        ▼
                 Print JSON to Console