import { invoke } from "@tauri-apps/api/core";
import { useCallback, useEffect, useState } from "react";
import { useNavigation } from "../lib/nav";
import { Clock } from "../lib/clock";
import useTime from "../lib/time";

function Item({ clock }: { clock: Clock }) {
  return (
    <div key={clock.id} className="flex flex-row items-center justify-between w-full h-20 px-0 py-4 border-b border-gray-500 border-solid last-of-type:border-0">
      <div className="flex flex-col justify-evenly h-full">
        <p className="text-sm">{clock.timezone}</p>
        <h2 className="text-4xl">{clock.city_name}</h2>
      </div>
      <div className="flex flex-row items-baseline h-full">
        <p className="text-6xl">{clock.time || ""}</p>
        <p className="text-4xl">{clock.meridian || ""}</p>
      </div>
    </div>
  );
}

function EditableItem({ clock }: { clock: Clock }) {
  return (
    <div key={clock.id} className="flex flex-row items-center justify-between w-full h-20 px-0 py-4 border-b border-gray-500 border-solid last-of-type:border-0">
      <div className="flex flex-col justify-evenly h-full">
        <div>
          <button onClick={() => console.log("delete is pending")}>
            Delete
          </button>
          <div>
            <p className="text-sm">{clock.timezone}</p>
            <h2 className="text-4xl">{clock.city_name}</h2>
          </div>
        </div>
      </div>
      <div className="flex flex-row items-baseline h-full">
        <button onClick={() => console.log("drag is pending")}>Drag</button>
      </div>
    </div>
  );
}

export default function ClockList() {
  const [clocks, setClocks] = useState<Clock[]>([]);
  const [loaded, setLoaded] = useState<boolean>(false);
  const navigation = useNavigation();
  const [mode, setMode] = useState<"list" | "edit">("list");
  const withTime = useTime(clocks);

  useEffect(() => {
    async function getClocks() {
      const data = await invoke<Clock[]>("get_clocks");
      setClocks(data);
      setLoaded(true);
    }

    getClocks();
  }, []);

  const add = useCallback(
    () => navigation.setScreen("clock-add"),
    [navigation],
  );
  const edit = useCallback(() => setMode("edit"), []);
  const done = useCallback(() => setMode("list"), []);

  if (!loaded) {
    return <p>Loading ...</p>;
  }

  if (clocks.length === 0) {
    return (
      <>
        <button className="bg-black text-inherit px-3 py-2 cursor-pointer outline-inherit rounded-md border border-solid border-gray-500 hover:opacity-30 focus:bg-black" onClick={() => navigation.setScreen("clock-add")}>
          Add
        </button>
        <p>No clocks found</p>
      </>
    );
  }

  if (mode === "edit") {
    return (
      <>
        <div className="flex flex-row items-center justify-between w-full mt-5">
          <button className="bg-black text-inherit px-3 py-2 cursor-pointer outline-inherit rounded-md border border-solid border-gray-500 hover:opacity-30 focus:bg-black" onClick={done}>Done</button>
          <button className="bg-black text-inherit px-3 py-2 cursor-pointer outline-inherit rounded-md border border-solid border-gray-500 hover:opacity-30 focus:bg-black" onClick={add}>Add</button>
        </div>
        {withTime.map((clock) => (
          <EditableItem key={clock.id} clock={clock} />
        ))}
      </>
    );
  }

  return (
    <>
      <div className="flex flex-row items-center justify-between w-full mt-5">
        <button className="bg-black text-inherit px-3 py-2 cursor-pointer outline-inherit rounded-md border border-solid border-gray-500 hover:opacity-30 focus:bg-black" onClick={edit}>Edit</button>
        <button className="bg-black text-inherit px-3 py-2 cursor-pointer outline-inherit rounded-md border border-solid border-gray-500 hover:opacity-30 focus:bg-black" onClick={add}>Add</button>
      </div>
      {withTime.map((clock) => (
        <Item key={clock.id} clock={clock} />
      ))}
    </>
  );
}

