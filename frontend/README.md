# XForce Analytics - Frontend

Yew WebAssembly frontend application.

## Structure

```
frontend/
├── src/
│   ├── components/   # Reusable components
│   ├── pages/        # Page components
│   ├── hooks/        # Custom hooks
│   ├── api/          # API client
│   └── main.rs       # Entry point
│
├── public/           # Static assets
├── styles/           # CSS styles
└── index.html        # HTML template
```

## Tech Stack

- **Yew 0.21** - Frontend framework
- **Yew Router** - Client-side routing
- **Gloo** - Web APIs
- **WASM** - WebAssembly target

## Development

```bash
# Install Trunk
cargo install trunk

# Install WASM target
rustup target add wasm32-unknown-unknown

# Run dev server
trunk serve --open
```

Frontend runs on http://localhost:8080

## Building

```bash
# Production build
trunk build --release
```

Output in `dist/` directory.

## Integration

Connects to backend API at `http://localhost:3000`

Configure in `.env`:
```
BACKEND_URL=http://localhost:3000
```
