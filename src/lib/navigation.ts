import { writable } from "svelte/store";

export type NavScreen = "clock-list" | "clock-add";

export const screen = writable<NavScreen>("clock-list");

export function setScreen(next: NavScreen) {
    screen.set(next);
}
