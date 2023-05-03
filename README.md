# Miraculous Kingdom

The Miraculous Kingdom is a Table Top Role Playing Deck Building Game. There are three main rules: Backstab, Decieve, and BlindSide. This is game is designed primarily to be deployed either on a home computer or in a home network via `docker compose up -d` on the root of the project.

The game is in active development and in its `0.1.1` version. But I have posted in a Roadmap.md for anyone intereste in the development.

## Installation
You will need to have a couple of things in order to start this service. Get the doceker compose file, and run `docker compose up -d` after changing the specified arguments in the `docker-compose.yml` file.

## Notes to myself and you.
Regarless of how this does in the TTRPG space this has been a great experience developing and learning using `axum` webserver with rust. I absolutely love the ecosystem that `axum` provides, and absolutely love the ergonomics of the entire project. On top of that I have been using svelte in stead of the original `NEXT.13` app that i was trying to make. In my opinion, `NEXT.13` did best in routing but not really in any other field for me. `Svelte` on the other hand had a great developer experience, a great development pipeline, and an decent routing solution. So overall Svelte was clearly the correct choice.

