<script lang="ts">
    import ClockComp from "../../components/Clock.svelte";
    import { Clock } from "../../models";

    const notConf: Clock = {
       clock_name: "Example",
       clock_desc: "This is an example of a Clock in Miraculous Kingdom",
       clock_char_secret: "notConf",
       clock_remaining: 3,
       clock_duration: 4,
       clock_conf: false
    } as Clock;


    const conf: Clock = {
       clock_name: "Example",
       clock_desc: "This is an example of a Clock in Miraculous Kingdom",
       clock_char_secret: "conf",
       clock_remaining: 3,
       clock_duration: 4,
       clock_conf: true
    } as Clock;


</script>

<div
    class="flex min-h-full py[] w-10/12 flex-col rounded-xl bg-slate-600/70 px-12 pt-6 text-justify align-top text-xl text-slate-300 backdrop-blur-lg"
>
    <div
        class="top-0 mb-8 flex flex-col rounded-xl bg-slate-800 p-8 text-7xl font-bold text-blue-600"
    >
       Clocks 
    </div>
    <div
        class="mx-2 flex h-full w-full flex-col justify-evenly rounded-lg bg-slate-300 dark:bg-black px-4 py-2 text-justify text-sm backdrop-blur"
    >
        The following is an example of the game&apos;s clock screens and this rules 
    </div>
    <div class="py-8">
        <ClockComp clock={notConf} secret="notConf"/>
    </div>
    <div class="py-8">
        <ClockComp clock={conf} secret="notConf"/>
    </div>
    <div
        class="mx-2 flex h-full w-full flex-col justify-evenly rounded-lg bg-slate-300 dark:bg-black px-4 py-2 text-justify text-sm backdrop-blur"
    >
        As you can see a clock is a modifier to how you play the game. These are accessible to all players unless if the Clock is marked as confidential. When marked as confidential, and the Clock is yours other players will know that the clock has been played but you are the only person who can see it after it is played.
    </div>
</div>
