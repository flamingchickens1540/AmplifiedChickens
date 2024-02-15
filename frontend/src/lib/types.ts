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
    match_key: MatchKey
    team_key: TeamKey
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
    stage_enum: StageEnum
    skill: number
    is_broke: boolean
    is_died: boolean
    notes: string
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

export type TeamEvent = {
    length: number
    width: number
    is_short: boolean
    drivetrain: DrivetrainEnum
    polish: number
    is_ground_intake: boolean
    is_chute_intake: boolean
    notes: string
}

export type StageEnum = "onstage" | "park" | "not attempted" | "failed"

export type DrivetrainEnum = "swerve" | "tank" | "other"

// Util Structs

export type TeamMatch = {
    number: string
    scout_name: string
    status: string
}
