export function formatTime12h(
    timezone: string,
    date: Date = new Date(),
): string {
    const options: Intl.DateTimeFormatOptions = {
        hour: "numeric",
        minute: "2-digit",
        hour12: true,
        timeZone: timezone,
    };
    const fmt = new Intl.DateTimeFormat(undefined, options);
    return fmt.format(date);
}
