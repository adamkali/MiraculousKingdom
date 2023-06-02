# Miraculous Kingdom

The Miraculous Kingdom is a Table Top Role Playing Deck Building Game. There are three main rules: Backstab, Decieve, and BlindSide. This is game is designed primarily to be deployed either on a home computer or in a home network via `docker compose up -d` on the root of the project.

The game is in active development and in its `0.9.0` version. But I have posted in a Roadmap.md for anyone intereste in the development. The issues page will also be updated for peridically for what is being worked on. Feel free to add an issue an omat you feel is needed!

Once we have determined that miraculous kingdom is in a state that people can play a game and not have any problems , evil released version 1.0.  1.0.0 is also marked in a milestone on GitHub.  So if you are curious when the full game will be released for the first time take a look at that.  However if anyone is interested and playing beforehand and beta testing we welcome you with open arms! Just remember that it will be in beta until version 1.0 its hit.

## Installation
You will need to have a couple of things in order to start this service. Get the doceker compose file, and run `docker compose up -d` after changing the specified arguments in the `docker-compose.yml` file that says `changeme`.

then go http://localhost:8000 and get to ensure everything is working.

### Additonal steps 
 Is recommended that you setup miraculous Kingdom to have an endpoint on your home network that you can access using https .  For example you can use nginx proxy manager 2 route the docker container prom `https://my-network/miraculous-kingdom`  to `localhost:8000`.  Having a full  tutorial on this is outside the scope of the installation process for miraculous Kingdom but I just want a point that for anyone who wants to have that as a dedicated endpoint on their home network. 

 ## Privacy
 Miraculous Kingdom is dedicated to user privacy.  We do not collect any information whatsoever if anyone finds code within Miraculous Kingdom's source code please contact us immediately as we did not put data collection code within its source code.  This is due to the philosophy of decentralized technology.

## Notes to myself and you.
Regarless of how this does in the TTRPG space this has been a great experience developing and learning using `axum` webserver with rust. I absolutely love the ecosystem that `axum` provides, and absolutely love the ergonomics of the entire project. On top of that I have been using svelte in stead of the original `NEXT.13` app that i was trying to make. In my opinion, `NEXT.13` did best in routing but not really in any other field for me. `Svelte` on the other hand had a great developer experience, a great development pipeline, and an decent routing solution. So overall Svelte was clearly the correct choice.

