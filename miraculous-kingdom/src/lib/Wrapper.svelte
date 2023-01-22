<script lang="ts">

    import { DIRECTORY } from './Constants';
    import { exists, BaseDirectory } from '@tauri-apps/api/fs';
    import {Intro} from './pages';

    let init: Promise<boolean> = exists(DIRECTORY + 'FIRSTTIME', { dir: BaseDirectory.AppData });

</script>

{#await init}
    <h1>Please stand by while we get things ready...</h1>
{:then res } 
    {#if res}
       <h1>This will hold the select</h1> 
    {:else}
       <Intro /> 
    {/if}
{:catch _err }
    <h1>Please try again...</h1>
    <h2>Error occured because of {_err}</h2>
{/await}
