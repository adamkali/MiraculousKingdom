<script lang="ts">
    import { ApiGameApiService, type GameCreation } from '../../models'
    import * as Components from '../../components'
    import { currentGame } from '../../store'

    let name: string
    let ruler_name: string

    const onClick = () => {
        const request: GameCreation = {
            game_name: name,
            game_ruler: ruler_name,
            game_num_players: 0,
        }

        ApiGameApiService.startGame(request)
            .then((resp) => {
                if (resp.success !== 'Succeeding') {
                    console.log({ err: resp.success.Failing.message })
                }
                currentGame.set(resp.data)
                window.location.href = `/games/create_character/${resp.data.game_pass}`
            })
            .catch((err) => console.log(err))
    }
</script>

<div
    class="flex min-h-screen w-10/12 flex-col items-center rounded-xl bg-slate-600/70 px-12 pt-6 text-center align-top text-xl text-slate-300 backdrop-blur-lg"
>
    <div
        class="top-0 mb-8 w-full rounded-xl bg-slate-800 p-8 text-7xl font-bold text-blue-600"
    >
        Start Game
    </div>
    <div class="justify-content flex w-1/2 flex-col">
        <div class="place-items-start">
            <Components.Input
                inputType={null}
                placeholder="Tywin Lanister"
                value={name}
                onChange={(value) => {
                    name = value
                }}
                label={'Country Name'}
            />
            <Components.Input
                inputType={null}
                placeholder="Godrick The Grafted"
                value={ruler_name}
                onChange={(value) => {
                    ruler_name = value
                }}
                label={'Ruler'}
            />
        </div>
        <div class="mt-24 flex w-1/3 flex-row items-center align-middle">
            <Components.Button {onClick}>Create a Game</Components.Button>
        </div>
    </div>
</div>
