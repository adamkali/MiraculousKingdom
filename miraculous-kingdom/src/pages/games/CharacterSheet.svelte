<script lang="ts">
    import * as Components from '../../components'
    import {
        MightEnum,
        type CharacterResponse,
        type GameInfo,
    } from '../../models'
    import { currentGame, gameCharacter } from '../../store'

    export let game: GameInfo
    export let character: CharacterResponse
</script>

<div
    class="flex min-h-screen w-10/12 flex-col items-center rounded-xl bg-slate-600/70 px-12 pt-6 text-center align-top text-xl text-slate-300 backdrop-blur-lg"
>
    <div
        class="top-0 mb-8 w-full rounded-xl bg-slate-800 p-8 text-5xl font-bold text-blue-600"
    >
        Miraculous Kingdom
    </div>
    <div class="grid grid-rows-5 gap-y-4">
        <div class="flex h-24 flex-row justify-center">
            <div class="mx-4">
                <Components.Input
                    label="Name"
                    placeholder=""
                    value={character.char_name}
                    onChange={(_value) => {}}
                    inputType="text"
                />
            </div>
            <div class="mx-4">
                <Components.Input
                    label="Class"
                    placeholder=""
                    value={character.char_class}
                    onChange={(_value) => {}}
                    inputType="text"
                />
            </div>
            <div class="mx-4">
                <Components.Input
                    label="Country Name"
                    placeholder=""
                    value={game.game_name}
                    onChange={(_value) => {}}
                    inputType="text"
                />
            </div>
            <div class="mx-4">
                <Components.Input
                    label="Ruler"
                    placeholder=""
                    value={game.game_ruler}
                    onChange={(_value) => {}}
                    inputType="text"
                />
            </div>
        </div>
        <div class="row-span-4 w-full justify-center">
            <Components.MightTable might={character.char_might} />
        </div>
        <Components.Button
            onClick={() => {
                currentGame.set(null)
                gameCharacter.set(null)
                window.location.href = '/'
            }}
        >
            Exit
        </Components.Button>
        <div class="grid grid-cols-9 w-full overflow-x-scroll">
            {#each character.char_deck as ability }
                <div class="mx-16 w-full h-full p-8">
                    <Components.Ability ability={ability} />
                </div>
            {/each} 
        </div>
    </div>
</div>
