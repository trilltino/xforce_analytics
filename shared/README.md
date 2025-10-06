# Shared Types

Type-safe models and DTOs shared between backend and frontend.

## Structure

```
shared/
├── src/
│   ├── models/       # Domain models
│   │   ├── user.rs
│   │   ├── project.rs
│   │   ├── category.rs
│   │   └── analytics.rs
│   │
│   ├── api_types/    # API request/response types
│   │   ├── auth.rs
│   │   ├── projects.rs
│   │   ├── analytics.rs
│   │   └── predictor.rs
│   │
│   └── errors/       # Error types
│       └── api_error.rs
```

## Usage

### In Backend
```rust
use shared::{User, LoginRequest, ProjectsResponse};
```

### In Frontend (WASM)
```rust
use shared::{User, LoginRequest, ProjectsResponse};
```

## Benefits

- **Type Safety**: Compile-time guarantees across stack
- **No Duplication**: Single source of truth
- **Serialization**: Automatic JSON conversion
- **Validation**: Shared validation logic
