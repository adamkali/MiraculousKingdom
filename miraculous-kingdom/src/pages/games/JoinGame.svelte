<script lang="ts">
    import { ApiGameApiService, type GameInfo } from '../../models'

    const getGames = async (): Promise<GameInfo[]> => {
        const response = await ApiGameApiService.getGames()
        if (response.success !== 'Succeeding') {
            throw new Error(response.success.Failing.message)
        }
        return response.data
    }

    const onClick = (pass: string): void => {
        window.location.href = `/games/create_character/${pass}`
    }
</script>

<div
    class="flex min-h-screen w-10/12 flex-col rounded-xl bg-slate-600/70 px-12 pt-6 text-center align-top text-xl text-slate-300 backdrop-blur-lg"
>
    <div
        class="top-0 mb-8 rounded-xl bg-slate-800 p-8 text-7xl font-bold text-blue-600"
    >
        Join a Game
    </div>
    {#await getGames()}
        <div>...waiting</div>
    {:then games}
        {#each games as game}
            <div class="relative">
                <div
                    class="lg absolute -inset-0.5 rounded bg-gradient-to-r from-fuchsia-600 to-blue-600 opacity-75 blur hover:opacity-100"
                />
                <div
                    on:click={() => onClick(game.game_pass)}
                    class="relative flex items-center rounded-lg bg-black/80 px-8 py-4 leading-none"
                >
                    <div
                        class="flex flex-row divide-x-2 divide-red-400 text-red-600"
                    >
                        <div>{game.game_name}</div>
                        <div>{game.game_ruler}</div>
                    </div>
                    <div class="flex flex-col text-slate-300">
                        <ul class="list-none">
                            {#each game.game_chars as char}
                                <li>{char}</li>
                            {/each}
                        </ul>
                    </div>
                </div>
            </div>
        {/each}
    {:catch err}
        <span>{err}</span>
    {/await}
</div>
