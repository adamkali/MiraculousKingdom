<script lang="ts">
    import { 
        type SeasonResponse as Season,
        type Ability,
        type Experience,
        type Token,
        type Clock,
    } from '../models';

    function isAbility(ability: any): ability is Ability {
        return ability.type === 'Ability'
    }

    function isClock(clock: any): clock is Clock {
        return clock.type === 'Clock'
    }

    function isExperience(experience: any): experience is Experience {
        return experience.type === 'Experience'
    }

    function isToken(token: any): token is Token {
        return token.type === 'Token'
    }

    export let ability: Ability;
</script>

<div class="flex w-full flex-row">
    <div class="group relative">
        <div
            class="lg absolute h-full w-full rounded bg-gradient-to-r from-fuchsia-600 to-blue-600 opacity-75 blur transition duration-150 ease-in-out group-hover:from-fuchsia-500 group-hover:to-blue-500 group-hover:opacity-100 group-hover:blur-xl"
        />
        <div
            class="mx-2 flex h-full w-full flex-row justify-evenly rounded-lg bg-black px-4 py-2 text-justify text-sm backdrop-blur"
        >
            <div class="ml-4 flex w-1/2 flex-col">
                <div class="text-2xl font-bold text-blue-400">
                    {ability.ability_name}
                </div>
                <p class="text-xl font-bold text-slate-400">
                    {ability.ability_desc}
                </p>
            </div>
            <div class="flex w-1/2 flex-col">
                {#each ability.ability_rewards as ab }
                    {#if ab !== 'None' && isAbility(ab)}
                        <div class="text-2xl font-bold text-blue-400">
                            Reward: Ability
                        </div>
                        <p class="text-xl font-bold text-slate-400">
                            {ab.ability_name}
                        </p>
                        <p class="text-xl font-bold text-slate-400">
                            {ab.ability_desc}
                        </p>
                    {:else if ab !== 'None' && isExperience(ab)}
                        <div class="text-2xl font-bold text-blue-400">
                            Reward: Experience
                        </div>
                        <p class="text-xl font-bold text-slate-400">
                            {ab.exp_type} + {ab.exp_amount}
                        </p>
                    {:else if ab !== 'None' && isClock(ab)}
                        <div class="text-2xl font-bold text-blue-400">
                            Reward: Clock
                        </div>
                        <p class="text-xl font-bold text-slate-400">
                            {ab.clock_name}
                        </p>
                        <p class="text-xl font-bold text-slate-400">
                            {ab.clock_desc}
                        </p>
                    {:else if ab !== 'None' && isToken(ab)}
                        <div class="flex w-1/2 flex-col">
                            <div class="text-2xl font-bold text-blue-400">
                                Reward: Token
                            </div>
                            <p class="text-xl font-bold text-slate-400">
                                {ab.token_type}
                            </p>
                            <p class="text-xl font-bold text-slate-400">
                                {ab.token_amount}
                            </p>
                        </div>
                    {:else}
                        <div class="flex w-1/2 flex-col">
                            <div class="text-2xl font-bold text-blue-400">
                                No Reward
                            </div>
                        </div>
                    {/if}
                {/each}
            </div>
        </div>
    </div>
</div>
