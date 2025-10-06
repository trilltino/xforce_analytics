# XForce Analytics - Backend

Production-grade Rust backend for SDF funding data explorer.

## Structure

```
backend/
├── libs/              # Reusable library crates
│   ├── lib-auth      # Authentication (password hashing, tokens)
│   ├── lib-core      # Core (models, DB, config)
│   ├── lib-utils     # Utilities (base64, time, validation)
│   ├── lib-web       # Web (middleware, error handling)
│   └── lib-rpc       # JSON-RPC (optional)
│
└── services/          # Application services
    └── web-server    # Main REST API server
```

## Tech Stack

- **Axum 0.8** - Web framework
- **SQLx 0.8** - Database (PostgreSQL)
- **Argon2** - Password hashing
- **Tower** - Middleware
- **Tracing** - Logging

## Running

```bash
# From project root
cargo run --package web-server

# Or with watch mode
cargo watch -x 'run --package web-server'
```

## API Endpoints

### Public
- `POST /api/auth/signup` - Register
- `POST /api/auth/login` - Login
- `POST /api/auth/logout` - Logout

### Protected (requires auth)
- `GET /api/projects` - List projects
- `GET /api/analytics` - Dashboard stats
- `POST /api/predictor` - Predict funding

See [API Documentation](../docs/api/) for details.
