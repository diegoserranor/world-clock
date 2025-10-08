import { useEffect, useMemo, useState } from "react";
import { Clock } from "./clock";

export default function useTime(clocks: Clock[]): Clock[] {
  const [now, setNow] = useState(() => new Date());

  useEffect(() => {
    const id = setInterval(() => setNow(new Date()), 60_000);
    return () => clearInterval(id);
  }, []);

  return useMemo(() => {
    return clocks.map((clock) => {
      const formatted = formatTime12h(clock.timezone).split(" ", 2);
      const time = formatted[0];
      const meridian = formatted[1];
      return {
        ...clock,
        time,
        meridian,
      };
    });
  }, [clocks, now]);
}

function formatTime12h(timezone: string): string {
  const options: Intl.DateTimeFormatOptions = {
    hour: "numeric",
    minute: "2-digit",
    hour12: true,
    timeZone: timezone,
  };
  const fmt = new Intl.DateTimeFormat(undefined, options);
  return fmt.format(new Date());
}

