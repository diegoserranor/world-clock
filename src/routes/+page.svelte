<script lang="ts">
    import Titlebar from "$lib/components/Titlebar.svelte";
    import ClockList from "$lib/screens/ClockList.svelte";
    import ClockAdd from "$lib/screens/ClockAdd.svelte";
    import { screen } from "$lib/navigation";
</script>

<div
    class="flex flex-col items-center justify-start min-h-screen px-14 py-7 bg-[#000000] text-white"
>
    <Titlebar />
    {#if $screen === "clock-list"}
        <ClockList />
    {:else if $screen === "clock-add"}
        <ClockAdd />
    {/if}
</div>
