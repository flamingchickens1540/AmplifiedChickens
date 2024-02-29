use crate::{
    model::{self, AppState},
    submit,
};
use axum::response::sse::Event;
use bimap::BiMap;
use serde::{Deserialize, Serialize};
use socketioxide::{
    extract::{Data, SocketRef, State},
    socket::Sid,
    SocketIo,
};
use tracing::{error, info};

pub struct QueueManager {
    robot_queue: Vec<(String, String)>, // team_key, team_color
    name_to_sid: BiMap<String, Sid>,
    pub matches: Vec<String>, // match keys matches.len() - 1 is the furthest
}

#[derive(Serialize)]
pub struct AdminRet {
    team_key: String,
    scout_name: String,
    match_key: String,
}

#[derive(Deserialize)]
pub struct NewMatchAuto {
    red_teams: Vec<String>,
    blue_teams: Vec<String>,
    match_key: String,
}

#[derive(Deserialize)]
pub struct NewMatchManual {
    red_teams: Vec<String>,
    blue_teams: Vec<String>,
    red_scouts: Vec<String>,
    blue_scouts: Vec<String>,
    match_key: String,
}

#[derive(Serialize)]
pub struct TeamMatchToScouts {
    team: String,
    match_key: String,
    // color: String, // TODO: Seperate red and blue teams for color consistency
}

impl QueueManager {
    pub fn new() -> Self {
        QueueManager {
            robot_queue: vec![],
            name_to_sid: BiMap::new(),
            matches: vec![],
        }
    }

    pub fn get_next_robot(&mut self) -> Option<(String, String)> {
        self.robot_queue.pop()
    }

    pub async fn free_scout(io: SocketIo, client_id: Sid) {
        let mut assigned_scouts = io.to("assigned_scouts").sockets().unwrap_or(vec![]);
        assigned_scouts.iter_mut().for_each(|socket| {
            // TODO: Right now this removes the scout in every sense if they have multiple instances
            if socket.id == client_id {
                socket.leave("assigned_scouts");
            }
        });
    }

    pub async fn assign_robots(
        &mut self,
        admin_socket: &SocketRef,
        robots: Vec<String>,
        scouts: &mut Vec<Sid>,
        state: &AppState,
        color: String,
    ) {
        let upstream = state.sse_upstream.lock().await;
        for team in robots.iter() {
            if robots.is_empty() {
                let mut robots_colors = robots
                    .into_iter()
                    .map(|robot| (robot, color.clone()))
                    .collect::<Vec<(String, String)>>();
                self.robot_queue.append(&mut robots_colors);
                info!("Out of scouts");
                return;
            }
            let scout = scouts.pop().expect("Scout queue is empty");
            match admin_socket.to(scout).emit("team_to_scout", team) {
                Ok(_) => info!("Team sent to scout"),
                Err(e) => error!("Error sending team to scout: {}", e),
            }

            let data = submit::SseReturn::DeQueuedScout(
                self.name_to_sid
                    .get_by_right(&scout)
                    .unwrap_or(&"Fake name".to_string())
                    .clone(),
            );

            match upstream.send(Ok(Event::default().data(
                serde_json::to_string(&data).expect("SseReturn struct not serializable"),
            ))) {
                Ok(_) => info!("Dequeued user"),
                Err(err) => error!("Failed to dequeue user: {}", err),
            }
        }
    }

    /// For manual matches, scouts should be the scout list, for auto matches, the scout should be pending_scouts
    pub async fn create_match(
        &mut self,
        admin_socket: &SocketRef,
        red_robots: Vec<String>,
        blue_robots: Vec<String>,
        red_scouts: &mut Vec<Sid>,
        blue_scouts: &mut Vec<Sid>,
        state: &model::AppState,
    ) {
        self.assign_robots(
            admin_socket,
            red_robots,
            red_scouts,
            state,
            "red".to_string(),
        );
        self.assign_robots(
            admin_socket,
            blue_robots,
            blue_scouts,
            state,
            "blue".to_string(),
        );
    }
}
pub async fn queue_scout_handler(
    socket: SocketRef,
    scout_name: Data<String>,
    state: State<model::AppState>,
) {
    let mut manager = state.queue_manager.lock().await;
    let socket_rooms = socket
        .rooms()
        .unwrap()
        .into_iter()
        .map(|room| room.to_string())
        .collect::<Vec<String>>();
    let is_assigned = socket_rooms.contains(&String::from("assigned_scouts"));
    let is_pending = socket_rooms.contains(&String::from("pending_scouts"));

    if is_assigned {
        error!("Assigned scout requested match");
        if is_pending {
            error!("")
        }
    } else if is_pending {
        error!("Pending scout requested match");
    }

    info!("Scout name: {}", scout_name.0);

    socket
        .join("pending_scouts")
        .expect("To be able to join a room");

    info!("Added {} to the pending_scouts room", scout_name.0);
    match manager.get_next_robot() {
        Some(robot) => {
            info!(
                "Robot avaliable\nRemoved {} from the pending_scouts room",
                scout_name.0
            );

            socket
                .leave("pending_scouts")
                .expect("To be able to leave a room");
            socket
                .join("assigned_scouts")
                .expect("To be able to join a room");
            match socket.emit("assign_team", robot.clone()) {
                Ok(_) => info!("Assignment Sent Back to Scout"),
                Err(err) => {
                    error!("Failed to Send Assignment Back to Scout");
                    panic!("{}", err)
                }
            };

            let num_of_queued_matches = f64::ceil(
                socket
                    .within("pending_scouts")
                    .sockets()
                    .unwrap_or(vec![])
                    .len() as f64
                    / 6.0,
            ) as usize;

            let curr_match = manager.matches[manager.matches.len() - num_of_queued_matches].clone();

            let admin_ret = AdminRet {
                team_key: robot.0,
                scout_name: scout_name.0,
                match_key: curr_match,
            };

            match socket
                .broadcast()
                .emit("team_match_assigned_admin", admin_ret)
            {
                Ok(_) => info!("Assignment Sent to Admin"),
                Err(err) => {
                    error!("Assignment Failed to Send to Admin\nError: {}", err);
                }
            };
        }
        None => {
            info!("No Robot avaliable");
            info!("Scout added to queue"); // The scout was already in the queue, they were just never removed
        }
    }
}

pub async fn dequeue_scout_handler(
    socket: SocketRef,
    scout_name: Data<String>,
    state: State<model::AppState>,
) {
    let rooms: Vec<String> = socket
        .rooms()
        .unwrap_or(vec![])
        .into_iter()
        .map(|room| room.to_string())
        .collect::<Vec<String>>();

    if !rooms.contains(&"pending_scouts".to_string()) {
        error!(
            "Attempted to dequeue scout: {} that was not pending\nScout was: {:?}",
            scout_name.0, rooms
        );
        return;
    }

    match socket.leave("pending_scouts") {
        Ok(_) => {}
        Err(err) => error!("Failed to remove scout from pending_scouts room: {}", err),
    }

    let data = submit::SseReturn::DeQueuedScout(scout_name.0);

    let upstream = state.sse_upstream.lock().await;
    match upstream.send(Ok(Event::default().data(
        serde_json::to_string(&data).expect("SseReturn struct not serializable"),
    ))) {
        Ok(_) => info!("Dequeued user"),
        Err(err) => error!("Failed to dequeue user: {}", err),
    }
}

pub async fn new_match_auto_handler(
    socket: SocketRef,
    match_info: Data<NewMatchAuto>,
    state: State<model::AppState>,
) {
    let mut manager = state.queue_manager.lock().await;

    manager.matches.push(match_info.0.match_key);

    let queued_scouts: Vec<Vec<Sid>> = socket
        .to("pending_scouts")
        .sockets()
        .unwrap_or(vec![])
        .into_iter()
        .map(|socket| socket.id)
        .collect::<Vec<Sid>>()
        .chunks(3)
        .map(|t| t.into())
        .collect();

    manager
        .create_match(
            &socket,
            match_info.0.red_teams,
            match_info.0.blue_teams,
            &mut queued_scouts[0].clone(), // this is fine because a mutable reference is just needed to appened scouts to
            &mut queued_scouts[1].clone(),
            state.0,
        )
        .await;
}

pub async fn new_match_manual_handler(
    socket: SocketRef,
    match_info: Data<NewMatchManual>,
    state: State<model::AppState>,
) {
    let mut manager = state.queue_manager.lock().await;

    manager.matches.push(match_info.0.match_key);

    let mut red_scouts = match_info
        .0
        .red_scouts
        .into_iter()
        .map(|scout| {
            manager
                .name_to_sid
                .get_by_left(&scout)
                .expect("Fatal: Scout SID is unknown")
                .clone()
        })
        .collect::<Vec<Sid>>();

    let mut blue_scouts = match_info
        .0
        .blue_scouts
        .into_iter()
        .map(|scout| {
            manager
                .name_to_sid
                .get_by_left(&scout)
                .expect("Fatal: Scout SID is unknown")
                .clone()
        })
        .collect::<Vec<Sid>>();

    manager
        .create_match(
            &socket,
            match_info.0.red_teams,
            match_info.0.blue_teams,
            &mut red_scouts,
            &mut blue_scouts,
            state.0,
        )
        .await;
}

pub async fn submit_team_match_handler(
    socket: SocketRef,
    team_match_data: Data<model::TeamMatch>,
    state: State<model::AppState>,
) {
    match socket.leave("assigned_scouts") {
        Ok(_) => info!("Scout left "),
        Err(err) => error!("Failed to remove scout from assigned scouts: {}", err),
    }
    submit::submit_team_match(&state, team_match_data.0).await;
}

pub async fn on_connect(
    socket: SocketRef,
    Data(username): Data<String>,
    State(state): State<AppState>,
) {
    let mut manager = state.queue_manager.lock().await;
    manager.name_to_sid.insert(username, socket.id);

    socket.on("queue_scout", queue_scout_handler);
    socket.on("dequeue_sout", dequeue_scout_handler);
    socket.on("new_match_auto", new_match_auto_handler);
    socket.on("new_match_manual", new_match_manual_handler);
    socket.on("submit_team_match", submit_team_match_handler);
}
