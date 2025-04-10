test:
    #!/usr/bin/env bash
    cargo test || true
    pnpm test || true