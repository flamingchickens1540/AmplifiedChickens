import type { EventKey } from "$lib/types";
import { type Writable, writable } from "svelte/store";

export const current_event_key: Writable<string> = writable('');