<script lang="ts">
    import * as Components from '../../components'
    import {
        MightEnum,
        type CharacterResponse,
        type GameInfo,
    } from '../../models'
    import { currentGame, gameCharacter } from '../../store';

    export let game: GameInfo;
    export let character: CharacterResponse
   
    console.log(game, character, $currentGame, $gameCharacter)
</script>

<div
    class="flex min-h-screen w-10/12 flex-col items-center rounded-xl bg-slate-600/70 px-12 pt-6 text-center align-top text-xl text-slate-300 backdrop-blur-lg"
>
    <div
        class="top-0 mb-8 w-full rounded-xl bg-slate-800 p-8 text-5xl font-bold text-blue-600"
    >
        Miraculous Kingdom
    </div>
    <div class="grid grid-rows-6 gap-y-4">
        <div class="grid-cols-4 gap-x-8">
            <Components.Input
                label="Name"
                placeholder=""
                value={character.char_name}
                onChange={(_value) => {}}
                inputType="text"
            />
            <Components.Input
                label="Class"
                placeholder=""
                value={character.char_class}
                onChange={(_value) => {}}
                inputType="text"
            />
            <Components.Input
                label="Country Name"
                placeholder=""
                value={game.game_name}
                onChange={(_value) => {}}
                inputType="text"
            />
            <Components.Input
                label="Ruler"
                placeholder=""
                value={game.game_ruler}
                onChange={(_value) => {}}
                inputType="text"
            />
        </div>
        <div class="flex w-full flex-row">
            <div class="mr-4">
                <Components.Input
                    inputType={'number'}
                    label={MightEnum.MILITARY}
                    placeholder="0"
                    value={character.char_might.might_military.stat_value.toString()}
                    onChange={(_value) => {}}
                />
            </div>
            <div class="mr-4">
                <Components.Input
                    inputType={'number'}
                    label={MightEnum.CULTURE}
                    placeholder="0"
                    value={character.char_might.might_culture.stat_value.toString()}
                    onChange={(_value) => {}}
                />
            </div>
            <div>
                <Components.Input
                    inputType={'number'}
                    label={MightEnum.SCIENCE}
                    placeholder="0"
                    value={character.char_might.might_science.stat_value.toString()}
                    onChange={(_value) => {}}
                />
            </div>
        </div>
        <div class="flex flex-row">
            <div class="mr-4">
                <Components.Input
                    inputType={'number'}
                    label={MightEnum.RELIGION}
                    placeholder="0"
                    value={character.char_might.might_religion.stat_value.toString()}
                    onChange={(_value) => {}}
                />
            </div>
            <div class="mr-4">
                <Components.Input
                    inputType={'number'}
                    label={MightEnum.DIPLOMACY}
                    placeholder="0"
                    value={character.char_might.might_diplomacy.stat_value.toString()}
                    onChange={(_value) => {}}
                />
            </div>
            <div>
                <Components.Input
                    inputType={'number'}
                    label={MightEnum.ESPIONAGE}
                    placeholder="0"
                    value={character.char_might.might_espionage.stat_value.toString()}
                    onChange={(_value) => {}}
                />
            </div>
        </div>
    </div>
</div>
