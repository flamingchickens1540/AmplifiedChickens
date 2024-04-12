import { writable, type Writable } from "svelte/store";
import { default_match_data, default_pit_data, default_auto_data, type TeamMatchData } from "$lib/types"
import { browser } from "$app/environment"
import type { AutoScoutData, Pit } from "$lib/types.ts"

export const manual: Writable<boolean> = writable(false);
export const current_event_key: Writable<string> = writable('');
export const team_color: Writable<"blue" | "red" | ""> = writable('');
export const count = writable(0);
export const pit: Writable<Pit> = writable(default_pit_data)

export const match_data: Writable<TeamMatchData> = writable(JSON.parse(JSON.stringify(default_match_data)))
export const auto_data: Writable<AutoScoutData> = writable(JSON.parse(JSON.stringify(default_auto_data)))
match_data.subscribe((val) => {
    if (browser) return (localStorage.match_data = JSON.stringify(val))
    else console.log("NO BROWSER")
})