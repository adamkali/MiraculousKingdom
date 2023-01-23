
<script lang="ts">
    import FaPlus from 'svelte-icons/fa/FaPlus.svelte'
    import FaMinus from 'svelte-icons/fa/FaMinus.svelte'
    import GiRollingDiceCup from 'svelte-icons/gi/GiRollingDiceCup.svelte'
    import type { TableData, TableRow } from "../types";
    import { MIGHTTABLE, TOKENTABLE } from "../Constants";

    export let table: TableData;
    export let save: (row: TableRow, data: TableData) => void;

    // On new character table will have a null value. this should load in tbale from constants.ts; otherwise, table will be set as the passed in prompt.
    if (table) {
        if (!table.name) {
            throw new Error("Table Name not provided! Table Components must provide data[\"name\"] in order to be valid.")
        }

        if (table.name == "Might" && !table.data) {
            table = MIGHTTABLE ;
        } else if (table.name == "Token" && !table.data) {
            table = TOKENTABLE ;
        } else {
            table.data = [ { "name": "Add New", "value": "Add Value" } as TableRow ] 
        }
    } else {
        throw new Error("Table must not be null or undefined to render.");
    }

    /**
    * handleIncrement handles the chnging the stats of a character
    */
    const handleIncrement = (name: string, inc: boolean) => {
        table.data.forEach((n) => {
            if (n.name === name) {
                let temp = +n.value;
                temp += inc ? 1 : -1;

                n.value = temp;
            }
        });
    }
</script>

<div>
    <div>{table.name}</div>
    <table class="text-cyan-400">
        {#each table.data as datum }
            <tr>
                {#if table.roll}
                    <div class="flex flex-row">
                        <div 
                            class="w-1/4 h-full"
                        >
                            <GiRollingDiceCup />
                        </div> 
                        <div class="flex flex-col justify-start w-1/2">
                            <div class="text-sm text-slate-400">{datum.name}</div>
                            <div>{datum.value}</div>
                        </div>
                        <div class="flex flex-col w-1/4">
                            <div 
                                class="h-1/2 border-b-slate-700"
                                on:click={() => handleIncrement(datum.name, true)}
                            >
                                <FaPlus />
                            </div> 
                            <div 
                                class="h-1/2"
                                on:click={() => handleIncrement(datum.name, false)}
                            >
                                <FaMinus />
                            </div>
                        </div> 
                    </div> 
                {:else if table.chng}
                    <div class="flex flex-row">
                        <div class="w-1/4">{" "}</div>
                        <div class="flex flex-col justify-start w-1/2">
                            <div class="text-sm text-slate-400">{datum.name}</div>
                            <div>{datum.value}</div>
                        </div>
                        <div class="flex flex-col w-1/4">
                            <div 
                                class="h-1/2 border-b-slate-700"
                                on:click={() => handleIncrement(datum.name, true)}
                            >
                                <FaPlus />
                            </div> 
                            <div 
                                class="h-1/2"
                                on:click={() => handleIncrement(datum.name, false)}
                            >
                                <FaMinus />
                            </div>
                        </div> 
                    </div> 
                {:else }
                    <div 
                        class="w-1/4 h-full"
                    >
                        {" "}
                    </div> 
                    <div class="flex flex-col justify-start w-1/2">
                        <div class="text-sm text-slate-400">{datum.name}</div>
                        <div>{datum.value}</div>
                    </div>
                    <div class="flex flex-col w-1/4">
                        {" "}
                    </div>
                {/if }
            </tr> 
        {/each}
    </table>
</div>
