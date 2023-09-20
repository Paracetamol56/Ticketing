# Contributing to the project

## Development setup

### Prerequisites
- Git: https://git-scm.com/
- Docker: https://www.docker.com/
- Rust: https://www.rust-lang.org/ (I advise you to use rustup)
- Cargo Shuttle: https://shuttle.rs/

### Clone the repository
```bash
git clone
```

### Build and run the backend
**Install the Shuttle CLI**
```bash
cargo binstall cargo-shuttle
cd ticketing/api/
```

**Edit the Secret.toml file**
```bash
cp Secret.sample.toml Secret.toml
vim Secret.toml # or the editor of your choice
```

**Run the backend**
```bash
cd ticketing/api/
cargo shuttle run
```

At this point, the backend should be running on port 8000. Note that Shuttle.rs uses automatic ressource provisioning, so you don't need to setup a MongoDB database, a docker container will be created for you.

### Build and run the frontend
**Install the dependencies**
```bash
cd ticketing/www/
bun install
```

**Run the frontend**
```bash
bun run dev
```
