export type Scout = {
    name: string,
    admin: boolean,
    noti: boolean
    status: MatchStatus
}

export type MatchStatus = "complete" | "pending" | "not_started"

export type TeamKey = `frc${number}`

export type MatchKey = `${EventKey}_${'qm' | 'qf' | 'sf' | 'f'}${number}`

export type EventKey = `${number}${string}`

export type TeamMatchData = {
    scout_id: string //
    match_key: string, //
    team_key: string, //
    is_fielded: boolean //
    is_leave_start: boolean //
    auto_speaker_succeed: number //
    auto_speaker_missed: number //
    auto_amp_succeed: number //
    auto_amp_missed: number //
    auto_piece_succeed: number //
    auto_piece_missed: number //
    tele_speaker_succeed: number //
    tele_speaker_missed: number //
    tele_amp_succeed: number //
    tele_amp_missed: number //
    trap_succeed: number //
    trap_missed: number //
    stage_enum: StageEnum //
    skill: number //
    notes: string
    is_broke: boolean
    is_died: boolean
}

export const default_match_data: TeamMatchData = {
    scout_id: "",
    match_key: "",
    team_key: "",
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
    stage_enum: "onstage", //did it
    skill: 0, //did it
    is_broke: false,
    is_died: false,
    notes: "" //did it
}

export type Team = {
    team_key: TeamKey
    nickname: string
}

export type Match = {
    match_key: MatchKey
    event_key: EventKey
    time: string
    red_1: TeamKey
    red_2: TeamKey
    red_3: TeamKey
    blue_1: TeamKey
    blue_2: TeamKey
    blue_3: TeamKey
}

export type Pit = {
    team_key: string
    length: number
    width: number
    is_short: boolean
    drivetrain: DrivetrainEnum
    polish: number
    is_ground_intake: boolean
    is_chute_intake: boolean
    notes: string
    is_camera: boolean
}

export const default_pit_data: Pit = {
    team_key: "",//oiajdsofuhasoidhufaosd
    length: 0,
    width: 0,
    is_short: true,//iuhoidhufaosdihufoaishudfoa
    polish: 3,//iuhoidhufaosdihufoaishudfoa
    is_ground_intake: false,//iuhoidhufaosdihufoaishudfoa
    drivetrain: "swerve", //iuhoidhufaosdihufoaishudfoa
    is_chute_intake: false,//iuhoidhufaosdihufoaishudfoa
    notes: "", //iajsdofjaosdifj
    is_camera: true
}

export type DrivetrainEnum = "swerve" | "tank" | "other"

export type StageEnum = "onstage" | "parked" | "notattempted" | "failed"

// Util Structs

export type TeamMatch = {
    team_key: string
    match_key: string
    scout_name: string
    status: MatchStatus 
}
