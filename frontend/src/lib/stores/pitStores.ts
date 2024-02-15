import type {Pit} from "$lib/types.ts"
import { writable, type Writable } from 'svelte/store';
export let pit: Writable<Pit> = writable({
    team_key: "frc2910",//oiajdsofuhasoidhufaosd
    length: 0,
    width: 0,
    weight: 0,
    is_short: true,//iuhoidhufaosdihufoaishudfoa
    drivetrain: "swerve", //iuhoidhufaosdihufoaishudfoa
    polish: 3,//iuhoidhufaosdihufoaishudfoa
    is_ground_intake: false,//iuhoidhufaosdihufoaishudfoa
    is_chute_intake: false,//iuhoidhufaosdihufoaishudfoa
    notes: "pretty bad" //iajsdofjaosdifj
})