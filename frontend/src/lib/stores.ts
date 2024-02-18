import { writable, type Writable } from "svelte/store";
import {default_match_data, default_pit_data, type TeamMatch, type TeamMatchData} from "$lib/types"

import type { Pit } from "$lib/types.ts"

export const current_event_key: Writable<string> = writable('');
export const team_color: Writable<"blue" | "red" | ""> = writable('');

export const pit: Writable<Pit> = writable(default_pit_data)

export const match_data: Writable<TeamMatchData> = writable(default_match_data)

