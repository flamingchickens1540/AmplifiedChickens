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
    scout_id: string
    match_key: string, 
    team_key: string, 
    location: string
    is_fielded: boolean
    is_leave_start: boolean
    auto_speaker_succeed: number
    auto_speaker_missed: number
    auto_amp_succeed: number
    auto_amp_missed: number
    auto_piece_succeed: number
    auto_piece_missed: number
    tele_speaker_succeed: number
    tele_speaker_missed: number
    tele_amp_succeed: number
    tele_amp_missed: number
    trap_succeed: number
    trap_missed: number
    stage: string
    skill: number
    is_broke: boolean
    is_died: boolean
    notes: string
}

export const default_match_data: TeamMatchData = {
    scout_id: "",
    match_key: "",
    team_key: "0",
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
    weight: number
    is_short: boolean
    drivetrain: DrivetrainEnum
    polish: number
    is_ground_intake: boolean
    is_chute_intake: boolean
    notes: string
}

export const default_pit_data: Pit = {
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
}

export type DrivetrainEnum = "swerve" | "tank" | "other"

// Util Structs

export type TeamMatch = {
    number: string
    scout_name: string
    status: MatchStatus 
}
