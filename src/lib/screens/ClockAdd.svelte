<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { setScreen } from "$lib/navigation";
    import type { City } from "$lib/city";

    let query = $state("");
    let results = $state<City[]>([]);

    const search = async (prefix: string) => {
        const data = await invoke<City[]>("search_cities", { prefix });
        results = data;
    };

    const add = async (name: string, timezone: string) => {
        await invoke("add_clock", { city_name: name, timezone });
    };
</script>

<div class="flex flex-row items-center justify-between w-full mt-5">
    <button onclick={() => setScreen("clock-list")}>Go back</button>
</div>

<form
    onsubmit={(event) => {
        event.preventDefault();
        search(query);
    }}
>
    <input type="text" bind:value={query} placeholder="Search for a city" />
    <button type="submit">Search</button>
</form>

{#each results as city (city.geonameid)}
    <button
        type="button"
        class="flex flex-row w-full py-3 px-3 border-b border-[#ffffff20] text-2xl text-left"
        onclick={() => add(city.name, city.timezone)}
    >
        {city.name}, {city.country_code}
    </button>
{/each}
