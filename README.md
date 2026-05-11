# ⚰️ Crypts n' Clues 🔍️

Medieval logical deduction game for friends. Inspired by Murdle. Investigate crime, collect evidence, solve mystery before your peers. Only one player gets paid—be the first to find truth.

## 🎲 Gameplay

Players take roles of investigators in a town plagued by murder. Gather clues, fill logic grids, and outsmart rivals.

- **Setup:** Random suspect files. Players pick 2 pieces of info (Suspect, Location, or Weapon) to start.
- **Deduction:** Private logic grids to track connections.
- **Public Board:** Reveal info to gain `Discovery` bonuses or choose new files.
- **Victory:** Correctly identify the killer, weapon, and location first.

## 🛠️ Tech Stack

### Backend

- **Rust** + **Axum**: High-performance web server.
- **SQLx** + **PostgreSQL**: Type-safe database queries and persistence.
- **Tokio**: Asynchronous runtime.
- **Serde**: JSON serialization.

### Frontend

- **Vue 3**: Reactive UI.
- **TypeScript**: Type safety.
- **Vite**: Ultra-fast build tool.
- **Tailwind CSS**: Modern styling.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (2024 edition)
- [Node.js](https://nodejs.org/) & [Bun](https://bun.sh/)
- [PostgreSQL](https://www.postgresql.org/)

### Server Setup

1. Navigate to `/server`.
2. Create `.env` from template (database URL, etc.).
3. Run migrations: `sqlx migrate run`.
4. Start server: `cargo run`.

### Client Setup

1. Navigate to `/client`.
2. Install dependencies: `bun install`.
3. Build web pages: `bun run build`.

## 📜 Documentation

See `/docs` for detailed rules and project notes.

- [Intro](./docs/intro.md)
- [Full Rules (Recommended)](./docs/rules.md)

## ⚖️ License

CC BY-NC-SA 4.0. See [LICENSE](./LICENSE) for details.
