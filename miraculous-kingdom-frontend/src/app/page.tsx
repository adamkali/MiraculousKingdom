import React from "react";

export default function Home() {
    return (
        <main className="flex min-h-screen min-w-full flex-col items-center text-slate-800 dark:text-slate-400">
            <div className="pt-64 text-6xl text-pink-600 dark:text-pink-400">
                <h1>Welcome to the Miraculous Kingdom!</h1>
            </div>
            <div className="flex flex-col px-96 py-60 text-justify text-2xl">
                <div className="pb-12">
                    <p>
                        Miraculous Kingdom is a Table Top Role Playing Game.
                        Think of Miraculous Kingdom (MK) as a drama like Game of
                        Thrones, The Wire, or The Last of Us. Your goal is to
                        plot, outwit, and betray your fellow players. At the
                        beginning of a round (
                        <b className="text-pink-600 dark:text-pink-400">
                            Season
                        </b>
                        ), roll a 20-sided dice. Who ever has the highest roll
                        will be the ruler for the season. From there, The ruler
                        for the season will have an event called from your
                        server. Then it is onto the races.
                    </p>
                </div>
                <div>
                    <p>
                        If you ever have a question, Checkout the rules on the
                        rules page. If that does not answer your questions, send
                        a pull request to{" "}
                        <a href="https://github.com/adamkali/MiraculousKingdom">
                            Our Gitub
                        </a>
                        . If you like what we are doing, consider giving the
                        projext a watch and star!
                    </p>
                </div>
            </div>
        </main>
    );
}
