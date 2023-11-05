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
    import {
        Abilities,
        Characters,
        Classes,
        Rules,
        MightRulePage,
    } from './pages/rules'
    import { Special } from './components'
    import { currentGame, gameCharacter } from './store'
    import { type CharacterResponse, type GameInfo } from './models'
    import Clocks from './pages/rules/Clocks.svelte'
    import { onMount } from 'svelte'
    import WaitingRoom from './pages/room/WaitingRoom.svelte'

    const backToHome = () => {
        window.location.href = '/'
    }
    let game = {} as GameInfo
    let char = {} as CharacterResponse
    $: gameInfo = game as GameInfo
    $: character = char as CharacterResponse

    onMount(() => {
        game = currentGame.get()
        char = gameCharacter.get()
    })
</script>

<main
    class="flex min-h-screen flex-col items-center justify-center bg-purple-400 p-4 font-victo-mono dark:bg-purple-700"
>
    <Router>
        <nav
            class="m-8 grid h-24 w-11/12 grid-cols-5 rounded-xl bg-black/75 px-8 py-2 text-2xl backdrop-blur-lg"
        >
            <div
                on:click={() => backToHome()}
                class="left-0 text-4xl text-fuchsia-600 dark:text-fuchsia-300"
            >
                Miraculous Kingdom
            </div>
            <div class="col-span-3 flex flex-row justify-center">
                <div
                    class="mr-16 h-8 flex-row items-center text-fuchsia-600 transition duration-150 hover:text-fuchsia-400 dark:text-fuchsia-300"
                >
                    <Link to="//games/start">
                        <GiDiceTwentyFacesTwenty />
                        <div>Start</div>
                    </Link>
                </div>
                <div
                    class="mr-16 h-8 flex-row items-center text-fuchsia-600 transition duration-150 hover:text-fuchsia-400 dark:text-fuchsia-300"
                >
                    <Link to="/games/join">
                        <GiHourglass />
                        <div>Join</div>
                    </Link>
                </div>
                {#if gameInfo.game_pass && character.secret}
                    <div
                        class="mr-16 h-8 flex-row items-center text-fuchsia-600 transition duration-150 hover:text-fuchsia-400 dark:text-fuchsia-300"
                    >
                        <Link to="/games/sheet">
                            <GiCardPlay />
                            <div>Sheet</div>
                        </Link>
                    </div>
                {/if}
                <div
                    class="mr-16 h-8 flex-row items-center text-fuchsia-600 transition duration-150 hover:text-fuchsia-400 dark:text-fuchsia-300"
                >
                    <Link to="/rules">
                        <GiOpenBook />
                        <div>Rules</div>
                    </Link>
                </div>
            </div>
            <div class="right-0 col-span-1 flex flex-row">
                <div
                    class="mr-8 h-12 text-fuchsia-600 transition duration-150 hover:text-fuchsia-400 dark:text-fuchsia-300"
                >
                    <Link to="https://github.com/adamkali/MiraculousKingdom">
                        <DiGithubAlt />
                    </Link>
                </div>
                <div
                    class="mr-8 h-12 text-fuchsia-600 transition duration-150 hover:text-fuchsia-400 dark:text-fuchsia-300"
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
            <Route path="clocks">
                <Clocks />
            </Route>
            <Route path="might">
                <MightRulePage />
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
        <Route path="/room/*">
            <Route path="/">
                <WaitingRoom />
            </Route>
        </Route>
    </Router>
</main>
