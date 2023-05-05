<script lang="ts">
    import FaYoutube from 'svelte-icons/fa/FaYoutube.svelte'
    import DiGithubAlt from 'svelte-icons/di/DiGithubAlt.svelte'
    import DiDocker from 'svelte-icons/di/DiDocker.svelte'
    import GiHourglass from 'svelte-icons/gi/GiHourglass.svelte'
    import GiOpenBook from 'svelte-icons/gi/GiOpenBook.svelte'
    import GiDiceTwentyFacesTwenty from 'svelte-icons/gi/GiDiceTwentyFacesTwenty.svelte'
    import { Router, Link, Route } from 'svelte-navigator'

    import Home from './pages/Home.svelte'
    import { StartGame, JoinGame, CreateCharacter } from './pages/games'
    import { Abilities, Characters, Classes } from './pages/rules'
    import { Special } from './components'

    const backToHome = () => {
        window.location.href = '/'
    }
</script>

<main
    class="mb-4 flex flex-col items-center justify-center bg-gradient-to-tr from-red-700 to-blue-700 font-victo-mono"
>
    <Router>
        <nav
            class="m-8 grid h-24 w-11/12 grid-cols-5 rounded-xl bg-black/75 p-4 px-4 py-2 text-2xl backdrop-blur-lg"
        >
            <div
                on:click={() => backToHome()}
                class="left-0 text-4xl text-red-600"
            >
                Miraculous Kingdom
            </div>
            <div class="col-span-3 flex flex-row justify-center">
                <div
                    class="mr-16 h-8 flex-row items-center text-blue-600 transition duration-150 hover:text-cyan-600"
                >
                    <Special href="/games/start">
                        <GiDiceTwentyFacesTwenty />
                        <div>Start</div>
                    </Special>
                </div>
                <div
                    class="mr-16 h-8 items-center text-blue-600 transition duration-150 hover:text-cyan-600"
                >
                    <Link to="/games/join">
                        <GiHourglass />
                        <div>Join</div>
                    </Link>
                </div>
                <div
                    class="mr-18 h-8 items-center text-blue-600 transition duration-150 hover:text-cyan-600"
                >
                    <Link to="/rules">
                        <GiOpenBook />
                        <div>Rules</div>
                    </Link>
                </div>
            </div>
            <div class="right-0 col-span-1 flex flex-row">
                <div
                    class="mr-8 h-12 text-red-600 transition duration-150 hover:text-fuchsia-600"
                >
                    <Link to="https://github.com/adamkali/MiraculousKingdom">
                        <DiGithubAlt />
                    </Link>
                </div>
                <div
                    class="h-12 text-red-600 transition duration-150 hover:text-fuchsia-600"
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
        </Route>
        <Route path="/games/*">
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
