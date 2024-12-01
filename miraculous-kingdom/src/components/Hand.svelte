<script lang="ts">
    import Ability from './Ability.svelte'
    import { type Ability as AbilityModel } from '../models'
    import { onMount } from 'svelte'

    export let hand: AbilityModel[] = [] as AbilityModel[]
    export let discard: (ability: AbilityModel) => Promise<void>
    export let send: (ability: AbilityModel) => Promise<void>
    let cardCount = 0
    let handDiv: HTMLDivElement
</script>

<div
    class="flex snap-x flex-row gap-4 overflow-y-auto scrollbar-track-black/80 scrollbar-thumb-black"
    bind:this={handDiv}
>
    {#each hand as ability, i}
        <div class="relative mb-8 snap-center p-8">
            <div
                class="absolute right-2 top-4 z-10 h-8 w-8 rounded-full bg-black text-gray-400 hover:scale-125"
                on:click={async () => await discard(ability)}
            >
                X
            </div>
            <Ability {ability} {send} />
        </div>
    {/each}
</div>
