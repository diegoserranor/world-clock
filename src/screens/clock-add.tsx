import { invoke } from "@tauri-apps/api/core";
import { useNavigation } from "../lib/nav";
import { useCallback, useState } from "react";
import { City } from "../lib/city";

export default function ClockAdd() {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<City[]>([]);
  const navigation = useNavigation();

  const search = useCallback(async (prefix: string) => {
    const data = await invoke<City[]>("search_cities", { prefix });
    setResults(data);
  }, []);

  const add = useCallback(async (name: string, timezone: string) => {
    await invoke("add_clock", { city_name: name, timezone });
  }, []);

  return (
    <>
      <div className="flex flex-row items-center justify-between w-full mt-5">
        <button onClick={() => navigation.setScreen("clock-list")}>
          Go back
        </button>
      </div>
      <form
        className=""
        onSubmit={(event) => {
          event.preventDefault();
          search(query);
        }}
      >
        <input
          type="text"
          value={query}
          onChange={(event) => setQuery(event.target.value)}
          placeholder="Search for a city"
        />
        <button type="submit">Search</button>
      </form>
      {results.map((city) => (
        <div
          key={city.geonameid}
          className="flex flex-row w-full py-3 px-3 border-b border-[#ffffff20] text-2xl"
          onClick={() => add(city.name, city.timezone)}
        >
          {city.name}, {city.country_code}
        </div>
      ))}
    </>
  );
}

