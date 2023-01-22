<script lang="ts">
    import CulturalAfficiando from './classes/CulturalAfficiando.svelte'
    import MilitaryAdvisor from './classes/MilitaryAdvisor.svelte'

    export let characterData;

    $: charClass = characterData.class;
</script>

<div>

{#if charClass == "001"}
   <CulturalAfficiando /> 
{/if}

{#if charClass == "002"}
   <MilitaryAdvisor /> 
{/if}

</div>
