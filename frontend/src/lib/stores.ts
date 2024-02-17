import { writable, type Writable } from "svelte/store";
import type {TeamMatch, TeamMatchData} from "$lib/types.ts"

import type { Pit } from "$lib/types.ts"

export const current_event_key: Writable<string> = writable('');

export const pit: Writable<Pit> = writable({
    team_key: "frc2910",//oiajdsofuhasoidhufaosd
    length: 0,
    width: 0,
    weight: 0,
    is_short: true,//iuhoidhufaosdihufoaishudfoa
    polish: 3,//iuhoidhufaosdihufoaishudfoa
    is_ground_intake: false,//iuhoidhufaosdihufoaishudfoa
    drivetrain: "swerve", //iuhoidhufaosdihufoaishudfoa
    is_chute_intake: false,//iuhoidhufaosdihufoaishudfoa
    notes: "" //iajsdofjaosdifj
})

export const match_data: Writable<TeamMatchData> = writable({
    match_key: "2024orsal_qm67",
    team_key: "frc2910" ,
    location: "Middle", //prematch done
    is_fielded: false,//prematch done
    is_leave_start: false,//auto done
    auto_speaker_succeed: 0, //auto done
    auto_speaker_missed: 0, //auto done
    auto_amp_succeed: 0, //auto done
    auto_amp_missed: 0, //auto done
    auto_piece_succeed: 0, //auto done
    auto_piece_missed: 0, //auto done
    tele_speaker_succeed: 0, //tele done
    tele_speaker_missed: 0, //tele done
    tele_amp_succeed: 0, //tele done
    tele_amp_missed: 0, //tele done
    trap_succeed: 0, //tele done
    trap_missed: 0, //tele done
    stage: "onstage", //did it
    skill: 0, //did it
    is_broke: true,
    is_died: true,
    notes: "" //did it
})
