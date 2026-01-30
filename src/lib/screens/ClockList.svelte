<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { setScreen } from "$lib/navigation";
    import type { Clock } from "$lib/clock";
    import { formatTime12h } from "$lib/time";

    let clocks = $state<Clock[]>([]);
    let loaded = $state(false);
    let mode = $state<"list" | "edit">("list");
    let now = $state(new Date());

    const add = () => {
        setScreen("clock-add");
    };
    const edit = () => {
        mode = "edit";
    };
    const done = () => {
        mode = "list";
    };
    const remove = async (clockId: string) => {
        try {
            const removed = await invoke<boolean>("delete_clock", {
                clock_id: clockId,
            });
            if (removed) {
                clocks = clocks.filter((clock) => clock.id !== clockId);
            }
        } catch (error) {
            console.error("Failed to delete clock", error);
        }
    };

    onMount(() => {
        void (async () => {
            const data = await invoke<Clock[]>("get_clocks");
            clocks = data;
            loaded = true;
        })();

        const id = setInterval(() => {
            now = new Date();
        }, 60_000);

        return () => clearInterval(id);
    });

    const withTime = $derived.by(() => {
        return clocks.map((clock) => {
            const formatted = formatTime12h(clock.timezone, now).split(" ", 2);
            const time = formatted[0];
            const meridian = formatted[1] ?? "";
            return {
                ...clock,
                time,
                meridian,
            };
        });
    });
</script>

{#if !loaded}
    <p>Loading ...</p>
{:else if clocks.length === 0}
    <button
        class="bg-black text-inherit px-3 py-2 cursor-pointer outline-inherit rounded-md border border-solid border-gray-500 hover:opacity-30 focus:bg-black"
        onclick={add}
    >
        Add
    </button>
    <p>No clocks found</p>
{:else if mode === "edit"}
    <div class="flex flex-row items-center justify-between w-full mt-5">
        <button
            class="bg-black text-inherit px-3 py-2 cursor-pointer outline-inherit rounded-md border border-solid border-gray-500 hover:opacity-30 focus:bg-black"
            onclick={done}
        >
            Done
        </button>
        <button
            class="bg-black text-inherit px-3 py-2 cursor-pointer outline-inherit rounded-md border border-solid border-gray-500 hover:opacity-30 focus:bg-black"
            onclick={add}
        >
            Add
        </button>
    </div>
    {#each withTime as clock (clock.id)}
        <div
            class="flex flex-row items-center justify-between w-full h-20 px-0 py-4 border-b border-gray-500 border-solid last-of-type:border-0"
        >
            <div class="flex flex-col justify-evenly h-full">
                <div>
                    <button onclick={() => remove(clock.id)}>Delete</button>
                    <div>
                        <p class="text-sm">{clock.timezone}</p>
                        <h2 class="text-4xl">{clock.city_name}</h2>
                    </div>
                </div>
            </div>
            <div class="flex flex-row items-baseline h-full">
                <button onclick={() => console.log("drag is pending")}
                    >Drag</button
                >
            </div>
        </div>
    {/each}
{:else}
    <div class="flex flex-row items-center justify-between w-full mt-5">
        <button
            class="bg-black text-inherit px-3 py-2 cursor-pointer outline-inherit rounded-md border border-solid border-gray-500 hover:opacity-30 focus:bg-black"
            onclick={edit}
        >
            Edit
        </button>
        <button
            class="bg-black text-inherit px-3 py-2 cursor-pointer outline-inherit rounded-md border border-solid border-gray-500 hover:opacity-30 focus:bg-black"
            onclick={add}
        >
            Add
        </button>
    </div>
    {#each withTime as clock (clock.id)}
        <div
            class="flex flex-row items-center justify-between w-full h-20 px-0 py-4 border-b border-gray-500 border-solid last-of-type:border-0"
        >
            <div class="flex flex-col justify-evenly h-full">
                <p class="text-sm">{clock.timezone}</p>
                <h2 class="text-4xl">{clock.city_name}</h2>
            </div>
            <div class="flex flex-row items-baseline h-full">
                <p class="text-6xl">{clock.time || ""}</p>
                <p class="text-4xl">{clock.meridian || ""}</p>
            </div>
        </div>
    {/each}
{/if}
