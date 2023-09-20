# Ticketing

## Overview
I realized a lot of people are asking for my attention, probably because I'm an extremely handsome person. This is nice but I don't have enough time to satisfy everyone. So I made this app (probably took longer to make it than to spend one hour with each person) to help me organize my time and to help you get my attention. I hope you'll enjoy it.

## Installation

**See the [development setup](CONTRIBUTING.md#development-setup) section of the contributing guide.**

## Technologies

<div align="center">
    <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/1024px-Rust_programming_language_black_logo.svg.png" height= 100px align="center" alt="Rust Logo" style="margin-right: 10px;">
    <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/6/60/Tokio_logo.svg/1200px-Tokio_logo.svg.png" height= 100px align="center" alt="Rust Logo" style="margin-right: 10px;">
    <img src="https://upload.wikimedia.org/wikipedia/fr/thumb/4/45/MongoDB-Logo.svg/1280px-MongoDB-Logo.svg.png" height= 100px align="center" alt="MongoDB Logo" style="margin-right: 10px;">
    <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/1/1b/Svelte_Logo.svg/1200px-Svelte_Logo.svg.png" height= 100px align="center" alt="Svelte Logo" style="margin-right: 10px;">
    <img src="https://raw.githubusercontent.com/shuttle-hq/shuttle/main/assets/logo-rectangle-transparent.png" height= 100px align="center" alt="Shuttle.rs Logo" style="margin-right: 10px;">
    <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/9/97/Netlify_logo_%282%29.svg/1200px-Netlify_logo_%282%29.svg.png" height= 100px align="center" alt="Netlify Logo">
</div>

### Backend
I choose to use the Rust programming language for the backend in combination with the Tokio runtime and the Axum framework.

- Rust: https://www.rust-lang.org/
- Tokio: https://tokio.rs/
- Axum: https://github.com/tokio-rs/axum

### Frontend
For the first time I decided to use Svelte for the frontend. To help me with the design I used the Tailwind CSS framework and Melt-ui. Icons are from Lucide.

- Svelte: https://svelte.dev/
- Tailwind CSS: https://tailwindcss.com/
- Melt-ui: https://www.melt-ui.com/
- Lucide: https://lucide.dev/

### Deployment
For deployment, I use only PaaS services. The frontend is deployed on Netlify and the backend on Shuttle.rs. The MongoDB database is also hosted on Shuttle as a shared database resource.

- Netlify: https://www.netlify.com/
- Shuttle.rs: https://shuttle.rs/
