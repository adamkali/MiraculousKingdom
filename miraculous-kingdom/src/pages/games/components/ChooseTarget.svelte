<script lang="ts">
    import {
        type Might,
        type SeasonResponse as Season,
        type Ability,
        type CharacterResponse as Character,
        ApiCharacterApiService,
    } from '../../../models'
    import * as Components from '../../../components'

    export let might: Might
    export let season: Season
    export let ability: Ability
    export let pass: string

    let character: Character

    $: characters = [] as Character[]

    const getCharacters = async () => {
        const res = await ApiCharacterApiService.getCharactersByGame(pass)
        if (res.success !== 'Succeeding') {
            throw new Error(
                'Failed to get characters: ' + res.success.Failing.message,
            )
        } else {
            characters = res.data
            return
        }
    }
</script>

<div class="grid-row-4 grid gap-8 p-4">
    <div class="w-full justify-center">
        <Components.MightTable {might} />
    </div>
    <div class="w-full justify-center">
        <Components.Season {season} />
    </div>
    <div class="w-full justify-center">
        <Components.AbilityWide {ability} />
    </div>
    <div class="w-full justify-center">
        <div class="flex flex-row justify-center">
            <div class="px-4">
                <Components.Input
                    label="Choose a Target Character"
                    value=""
                    inputType="text"
                    onChange={() => {}}
                    disabled={true}
                    placeholder=""
                />
            </div>
            {#await getCharacters()}
                waiting...
            {:then}
                <div class="px-4">
                    <select
                        class=" h-24 w-full rounded-lg bg-black p-4 text-xl text-slate-300"
                        on:change={(e) => {
                            character = characters.find(
                                (c) => c.secret === e.currentTarget.value,
                            )
                        }}
                    >
                        {#each characters as c}
                            <option value={c.secret}>
                                {c.char_name}
                            </option>
                        {/each}
                    </select>
                </div>
            {:catch error}
                <p>{error.message}</p>
            {/await}
        </div>
    </div>
</div>
