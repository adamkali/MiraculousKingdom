<script lang="ts">
    import Special from '../../components/Special.svelte'
    import Ability from '../../components/Ability.svelte'
    import { ApiClassApiService, type ClassResponse } from '../../models'

    const getClasses = async (): Promise<ClassResponse[]> => {
        let response = await ApiClassApiService.getClasses()
        if (response.success != 'Succeeding') {
            throw new Error(
                'Could not get Classes from the Database: ' +
                    response.success.Failing.message,
            )
        }
        return response.data
    }
</script>

<div
    class="item-center flex w-10/12 flex-col justify-center rounded-xl bg-slate-600/70 px-4 py-16 text-center text-xl text-slate-300 backdrop-blur-lg"
>
    <h1 class="top-0 text-5xl text-red-600">Character Class</h1>
    <p class="mb-4">
        When you create a <Special href="/rules/characters">Character</Special>,
        you get to choose a <Special href="/rules/classes">Class</Special>. Each
        class has its own special passive and its own <Special
            href="/rules/abilities">Abilities</Special
        >. Each of these abilities are are made for the class, and they allow
        each class to utilize a different play style. Read them carefully and
        try to make use of the game's other
    </p>
    <h2 class="text-4xl text-fuchsia-600">Avaliable Classes</h2>
    {#await getClasses()}
        <p>Waiting on the server</p>
    {:then aClasses}
        {#each aClasses as aClass}
            <p>
                <b class="text-2xl text-cyan-400">{aClass.class_name}</b>{' ' +
                    aClass.class_desc}
            </p>
            <p>
                <b class="text-2xl text-cyan-400"
                    >Passive
                </b>{aClass.class_perks}
            </p>
            <h4 class="text-2xl text-cyan-400">Abilities</h4>
            <div
                class="mx-4 my-8 grid w-fit grid-cols-4 grid-rows-2 justify-center gap-x-8 gap-y-12 align-middle"
            >
                {#each aClass.class_abilities as ability}
                    <Ability {ability} />
                {/each}
            </div>
        {/each}
    {:catch err}
        <p>{err}</p>
    {/await}
</div>
