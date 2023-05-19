<script lang="ts">
    import Ability from './Ability.svelte';
    import { type Ability as AbilityModel } from '../models';
    import { onMount } from 'svelte';

    export let hand: AbilityModel[] = [] as AbilityModel[];
    let cardCount = 0
    let handDiv: HTMLDivElement

</script>

<div class="flex flex-row gap-4 w-[90rem] overflow-y-auto" bind:this={handDiv}>
    {#each hand as ability, i}
        <div class="relative p-8 mb-8">
            <div class="absolute z-10 w-8 h-8 top-4 right-2 rounded-full bg-black text-gray-400 hover:scale-125"> 
                X 
            </div>
            <Ability ability={ability}/>   
        </div>
    {/each}
</div>
