<script lang="ts">
    import {
        ApiCharacterApiService,
        ApiGameApiService,
        type GameInfo,
    } from '../../models'
    import { currentGame, gameCharacter } from '../../store'
    import * as Components from '../../components'
    import { onMount } from 'svelte'

    onMount(() => {
        currentGame.set({} as GameInfo)
    })

    const getGames = async (): Promise<GameInfo[]> => {
        const response = await ApiGameApiService.getGames()
        if (response.success !== 'Succeeding') {
            throw new Error(response.success.Failing.message)
        }
        return response.data
    }

    const handleJoinNew = (g: GameInfo): void => {
        currentGame.set(g)
        window.location.href = `/ui/games/create_character/${g.game_pass}`
    }

    const handleJoinExisting = (secret: string, g: GameInfo): void => {
        ApiCharacterApiService.getCharacterForGame(secret, g.game_pass)
            .then((res) => {
                if (res.success !== 'Succeeding') {
                    console.log({
                        'error code': res.success.Failing.status_code,
                        'error message': res.success.Failing.message,
                    })
                } else {
                    if (res.data.char_name !== 'No Character') {
                        gameCharacter.set(res.data)
                        currentGame.set(g)
                        window.location.href = `/ui/games/sheet`
                    }
                }
            })
            .catch((err) => {
                console.log({
                    message: err,
                })
            })
    }

    const handleSecretChange = (value: string) => {
        secret = value
    }

    let secret: string
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
            <div class="mb-8 flex h-36 flex-row items-center justify-center">
                <div class="h-full w-full p-4">
                    <Components.Input
                        label={game.game_name}
                        value={game.game_ruler}
                        onChange={(_value) => {}}
                        disabled={true}
                        inputType="text"
                        placeholder=""
                    />
                </div>
                <div class="h-full w-full translate-y-8 items-center p-4">
                    <Components.Button onClick={() => handleJoinNew(game)}>
                        Join With New Character
                    </Components.Button>
                </div>
                <div class="h-full w-full p-4">
                    <Components.InputButton
                        label="Join With Existing Character"
                        value={secret}
                        onClick={() => handleJoinExisting(secret, game)}
                        onChange={(value) => handleSecretChange(value)}
                        placeholder="WireWatcher11235!"
                    />
                </div>
            </div>
        {/each}
    {:catch err}
        <span>{err}</span>
    {/await}
</div>
