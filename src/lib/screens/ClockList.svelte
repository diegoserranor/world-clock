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
    let draggingId = $state<string | null>(null);

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
        const ordered = [...clocks].sort((a, b) => a.order - b.order);
        return ordered.map((clock) => {
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

    const startDrag = (event: DragEvent, clockId: string) => {
        draggingId = clockId;
        if (event.dataTransfer) {
            event.dataTransfer.setData("text/plain", clockId);
            event.dataTransfer.effectAllowed = "move";
            if (event.currentTarget instanceof HTMLElement) {
                event.dataTransfer.setDragImage(
                    event.currentTarget,
                    event.currentTarget.offsetWidth / 2,
                    event.currentTarget.offsetHeight / 2
                );
            }
        }
    };

    const clearDrag = () => {
        draggingId = null;
    };

    const allowDrop = (event: DragEvent) => {
        event.preventDefault();
        if (event.dataTransfer) {
            event.dataTransfer.dropEffect = "move";
        }
    };

    const dropOn = async (event: DragEvent, targetId: string) => {
        event.preventDefault();
        const draggedId =
            draggingId ||
            event.dataTransfer?.getData("text/plain") ||
            null;
        if (!draggedId || draggedId === targetId) {
            return;
        }
        const current = [...clocks].sort((a, b) => a.order - b.order);
        const fromIndex = current.findIndex((clock) => clock.id === draggedId);
        const toIndex = current.findIndex((clock) => clock.id === targetId);
        if (fromIndex === -1 || toIndex === -1) {
            return;
        }
        const [moved] = current.splice(fromIndex, 1);
        current.splice(toIndex, 0, moved);
        const updated = current.map((clock, index) => ({
            ...clock,
            order: index,
        }));
        clocks = updated;
        try {
            const persisted = await invoke<Clock[]>("reorder_clocks", {
                order: updated.map((clock) => clock.id),
            });
            clocks = persisted;
        } catch (error) {
            console.error("Failed to reorder clocks", error);
        } finally {
            clearDrag();
        }
    };
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
            class="flex flex-row items-center justify-between w-full min-h-20 px-0 py-4 border-b border-gray-500 border-solid last-of-type:border-0"
            ondragover={allowDrop}
            ondrop={(event) => dropOn(event, clock.id)}
            role="listitem"
            draggable="true"
            ondragstart={(event) => startDrag(event, clock.id)}
            ondragend={clearDrag}
            class:opacity-60={draggingId === clock.id}
        >
            <div class="flex flex-col justify-evenly h-full">
                <div>
                    <div>
                        <p class="text-sm">{clock.timezone}</p>
                        <h2 class="text-4xl">{clock.city_name}</h2>
                    </div>
                </div>
            </div>
            <div class="flex flex-row items-baseline h-full justify-end gap-3">
                <button onclick={() => remove(clock.id)}>Delete</button>
                <button>Drag</button>
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
            class="flex flex-row items-center justify-between w-full min-h-20 px-0 py-4 border-b border-gray-500 border-solid last-of-type:border-0"
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
