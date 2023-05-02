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
    class="flex w-10/12 flex-col justify-center rounded-xl bg-slate-600/70 px-4 py-16 text-justify text-slate-300 backdrop-blur-lg"
>
    <h1 class="top-0 text-3xl text-red-600">Character Class</h1>
    <p>
        When you create a <Special href="/rules/character">Character</Special>,
        you get to choose a <Special href="/rules/class">Class</Special>. Each
        class has its own special passive and its own <Special
            href="/rules/ability">Abilities</Special
        >. Each of these abilities are are made for the class.
    </p>
    <h2 class="text-2xl text-fuchsia-600">Avaliable Classes</h2>
    {#await getClasses()}
        <p>Waiting on the server</p>
    {:then aClasses}
        {#each aClasses as aClass}
            <h3 class="text-xl text-cyan-400">{aClass.class_name}</h3>
            <p>{aClass.class_desc}</p>
            <h4 class="text-lg text-cyan-400">Passive</h4>
            <p>{aClass.class_perks}</p>
            <h4 class="text-lg text-cyan-400">Abilities</h4>
            <div class="flex-row justify-start overflow-x-scroll align-middle">
                {#each aClass.class_abilities as ability}
                    <Ability {ability} />
                {/each}
            </div>
        {/each}
    {:catch err}
        <p>{err}</p>
    {/await}
</div>
