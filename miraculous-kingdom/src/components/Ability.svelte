<script lang="ts">
    import { type Ability, MightEnum, RollTier } from '../models'
    import GiWinchesterRifle from 'svelte-icons/gi/GiWinchesterRifle.svelte'
    import GiFleurDeLys from 'svelte-icons/gi/GiFleurDeLys.svelte'
    import GiMaterialsScience from 'svelte-icons/gi/GiMaterialsScience.svelte'
    import GiCrossShield from 'svelte-icons/gi/GiCrossShield.svelte'
    import GiSecretBook from 'svelte-icons/gi/GiSecretBook.svelte'
    import GiSpy from 'svelte-icons/gi/GiSpy.svelte'

    export let ability: Ability
</script>

<div
    class="mx-2 flex h-64 w-32 flex-col justify-center overflow-y-scroll rounded-lg bg-slate-800/70 px-4 py-2 backdrop-blur"
>
    <div class="mb-2 text-lg text-fuchsia-600">
        {ability.ability_name}
    </div>
    <p class="mb-2">
        {ability.ability_desc}
    </p>
    {#if ability.ability_clock}
        <p class="mb-2">
            {ability.ability_clock}
        </p>
    {/if}
    <ul class="list-none">
        <li>
            <div class="flex-row items-center">
                {#if ability.ability_unlock.might == MightEnum.MILITARY}
                    <span class="h-8 text-red-600"><GiWinchesterRifle /></span>
                {:else if ability.ability_unlock.might == MightEnum.CULTURE}
                    <span class="h-8 text-purple-600"><GiFleurDeLys /></span>
                {:else if ability.ability_unlock.might == MightEnum.RELIGION}
                    <span class="h-8 text-pink-600"><GiCrossShield /></span>
                {:else if ability.ability_unlock.might == MightEnum.SCIENCE}
                    <span class="h-8 text-blue-600"><GiMaterialsScience /></span
                    >
                {:else if ability.ability_unlock.might == MightEnum.DIPLOMACY}
                    <span class="h-8 text-yellow-600"><GiSecretBook /></span>
                {:else if ability.ability_unlock.might == MightEnum.ESPIONAGE}
                    <span class="h-8 text-yellow-600"><GiSpy /></span>
                {/if}
                {ability.ability_unlock.unlock}
            </div>
        </li>
        {#if ability.ability_unlock.roll_tier === RollTier.FAIL}
            <li class="rounded-xl bg-white text-black">
                {ability.ability_unlock.roll_tier}
            </li>
        {:else if ability.ability_unlock.roll_tier === RollTier.BAD}
            <li class="rounded-xl bg-green-500 px-2 py-1 text-black">
                {ability.ability_unlock.roll_tier}
            </li>
        {:else if ability.ability_unlock.roll_tier === RollTier.NEUTRAL}
            <li class="w-fit rounded-xl bg-blue-500 px-2 py-1 text-black">
                {ability.ability_unlock.roll_tier}
            </li>
        {:else if ability.ability_unlock.roll_tier === RollTier.GOOD}
            <li class="w-fit rounded-xl bg-purple-700 px-2 py-1 text-white">
                {ability.ability_unlock.roll_tier}
            </li>
        {:else if ability.ability_unlock.roll_tier === RollTier.GREAT}
            <li
                class="w-fit rounded-xl bg-orange-600 px-2 py-1 text-yellow-200"
            >
                {ability.ability_unlock.roll_tier}
            </li>
        {:else if ability.ability_unlock.roll_tier === RollTier.FANTASTIC}
            <li class="w-fit rounded-xl bg-red-600 px-2 py-1 text-blue-300">
                {ability.ability_unlock.roll_tier}
            </li>
        {:else}
            <li>{' '}</li>
        {/if}
    </ul>
</div>
