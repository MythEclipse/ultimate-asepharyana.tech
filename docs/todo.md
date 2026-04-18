# Active Backlog

- [ ] Migrate Docker image build and deployment to Nix outputs
  - Scope: update `flake.nix` to generate Nix-native Docker image outputs for all services.
  - Acceptance criteria:
    - `nix build .#docker-rust` and `nix build .#docker-elysia` are available from the flake.
    - Extend Nix Docker outputs for `nextjs`, `solidjs`, `leptos`, and `visuals`.
    - Update `.github/workflows/docker-build-push.yml` to build and push Docker images from Nix outputs.
    - Document recommended Nix Docker build and deploy commands in `README.md`.
  - Risk: 45

