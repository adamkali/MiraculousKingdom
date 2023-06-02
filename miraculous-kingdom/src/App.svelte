<script lang="ts">
    import FaYoutube from 'svelte-icons/fa/FaYoutube.svelte'
    import DiGithubAlt from 'svelte-icons/di/DiGithubAlt.svelte'
    import DiDocker from 'svelte-icons/di/DiDocker.svelte'
    import GiHourglass from 'svelte-icons/gi/GiHourglass.svelte'
    import GiOpenBook from 'svelte-icons/gi/GiOpenBook.svelte'
    import GiDiceTwentyFacesTwenty from 'svelte-icons/gi/GiDiceTwentyFacesTwenty.svelte'
    import GiCardPlay from 'svelte-icons/gi/GiCardPlay.svelte'
    import { Router, Link, Route } from 'svelte-navigator'

    import Home from './pages/Home.svelte'
    import {
        StartGame,
        JoinGame,
        CreateCharacter,
        CharacterSheet,
    } from './pages/games'
    import { Abilities, Characters, Classes, Rules } from './pages/rules'
    import { Special } from './components'
    import { currentGame, gameCharacter } from './store'
    import { type CharacterResponse, type GameInfo } from './models'

    const backToHome = () => {
        window.location.href = '/'
    }

    console.log($currentGame, $gameCharacter)
</script>

<main
    class="p-8 flex flex-col items-center justify-center font-victo-mono bg-purple-400 dark:bg-purple-700"
>
    <Router>
        <nav
            class="m-8 grid h-24 w-11/12 grid-cols-5 rounded-xl bg-black/75 p-4 px-4 py-2 text-2xl backdrop-blur-lg"
        >
            <div
                on:click={() => backToHome()}
                class="left-0 text-4xl text-fuchsia-600 dark:text-fuchsia-300"
            >
                Miraculous Kingdom
            </div>
            <div class="col-span-3 flex flex-row justify-center">
                <div
                    class="mr-16 h-8 flex-row items-center text-fuchsia-600 dark:text-fuchsia-300 transition duration-150 hover:text-fuchsia-400"
                >
                    <Link to="/games/start">
                        <GiDiceTwentyFacesTwenty />
                        <div>Start</div>
                    </Link>
                </div>
                <div
                    class="mr-16 h-8 flex-row items-center text-fuchsia-600 dark:text-fuchsia-300 transition duration-150 hover:text-fuchsia-400"
                >
                    <Link to="/games/join">
                        <GiHourglass />
                        <div>Join</div>
                    </Link>
                </div>
                {#if !(!$currentGame.game_pass && !$gameCharacter.secret)}
                    <div
                        class="mr-16 h-8 flex-row items-center text-fuchsia-600 dark:text-fuchsia-300 transition duration-150 hover:text-fuchsia-400"
                    >
                        <Link to="/games/sheet">
                            <GiCardPlay />
                            <div>Sheet</div>
                        </Link>
                    </div>
                {/if}
                <div
                    class="mr-16 h-8 flex-row items-center text-fuchsia-600 dark:text-fuchsia-300 transition duration-150 hover:text-fuchsia-400"
                >
                    <Link to="/rules">
                        <GiOpenBook />
                        <div>Rules</div>
                    </Link>
                </div>
            </div>
            <div class="right-0 col-span-1 flex flex-row">
                <div
                    class="mr-8 h-12 text-fuchsia-600 dark:text-fuchsia-300 transition duration-150 hover:text-fuchsia-400"
                >
                    <Link to="https://github.com/adamkali/MiraculousKingdom">
                        <DiGithubAlt />
                    </Link>
                </div>
                <div
                    class="mr-8 h-12 text-fuchsia-600 dark:text-fuchsia-300 transition duration-150 hover:text-fuchsia-400"
                >
                    <Link
                        to="https://hub.docker.com/repository/docker/adamkali/miraculous-kingdom/general"
                    >
                        <DiDocker />
                    </Link>
                </div>
                <!--<Link 
                    class="text-red-600 transition duration-150 hover:text-fuchsia-600"
                    to="/"
                >
                    <FaYoutube /> 
                </Link>-->
            </div>
        </nav>
        <Route path="/">
            <Home />
        </Route>
        <Route path="/rules/*">
            <Route path="abilities">
                <Abilities />
            </Route>
            <Route path="classes">
                <Classes />
            </Route>
            <Route path="characters">
                <Characters />
            </Route>
            <Route path="/">
                <Rules />
            </Route>
        </Route>
        <Route path="/games/*">
            <Route path="sheet">
                <CharacterSheet />
            </Route>
            <Route path="start">
                <StartGame />
            </Route>
            <Route path="create_character/:pass" let:params>
                <CreateCharacter pass={params.pass} />
            </Route>
            <Route path="join">
                <JoinGame />
            </Route>
        </Route>
    </Router>
</main>
