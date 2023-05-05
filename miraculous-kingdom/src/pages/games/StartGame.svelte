<script lang="ts">
    import { ApiGameApiService, type GameCreation } from '../../models'
    import * as Components from '../../components'

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
                window.location.href = `/games/create_character/${resp.data}`;
            })
            .catch((err) => console.log(err))
    }
</script>

<div
    class="flex min-h-screen w-10/12 flex-col rounded-xl bg-slate-600/70 px-12 pt-6 text-center align-top text-xl text-slate-300 backdrop-blur-lg"
>
    <div
        class="top-0 mb-8 rounded-xl bg-slate-800 p-8 text-7xl font-bold text-blue-600"
    >
        Start Game
    </div>
    <div class="flex flex-col justify-content w-1/2">
        <div class="place-items-start">
            <Components.Input
                placeholder="Tywin Lanister"
                value={name}
                onChange={(value) => {
                    name = value
                }}
                label={'Name'}
            />
            <Components.Input
                placeholder="Godrick The Grafted"
                value={ruler_name}
                onChange={(value) => {
                    ruler_name = value
                }}
                label={'Ruler'}
            />
        </div>
        <div class="place-items-end w-1/3 flex flex-row justify-center mt-24">
            <Components.Button {onClick}>Create a Game</Components.Button>
        </div>
    </div>
</div>
