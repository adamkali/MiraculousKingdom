<script lang="ts">
    import { Button } from "../../components";

</script>


<div
    class="flex min-h-screen w-10/12 flex-col rounded-xl bg-slate-600/70 px-12 pt-6 text-center align-top text-xl text-slate-300 backdrop-blur-lg"
>
    <div
        class="top-0 mb-8 rounded-xl bg-slate-800 p-8 text-7xl font-bold text-blue-600"
    >
        Choose a Staring Point:
    </div>
    <div class="grid grid-rows-2">
        <Button onClick={() => { window.location.href = "/game/join";}}>
            Join
        </Button>
        <Button onClick={() => { window.location.href = "/game/start";}}>
            Start
        </Button>
    </div>
</div>
