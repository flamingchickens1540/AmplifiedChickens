use crate::{error, info, SocketIoLayer};
use bimap::BiMap;
use serde::{Deserialize, Serialize};
use socketioxide::{
    extract::{Data, SocketRef, State},
    socket::{DisconnectReason, Sid},
    SocketIo,
};
use std::sync::Mutex;
#[derive(Deserialize, Debug)]
struct TeamMatchData {
    _data: String, // This would be actual teamData maybe, not sure exactly
}

/// This struct is received from the admin when a match is created
#[derive(Deserialize)]
struct Match {
    red_robots: Vec<String>,
    blue_robots: Vec<String>,
    match_key: String,
}

/// This struct is sent back to a scout
#[derive(Serialize)]
struct ScoutReturn {
    team_key: String,
    match_key: String,
}

/// This struct is send back to admin
#[derive(Serialize)]
struct AdminReturn {
    team_key: String,
    name: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
struct Scout {
    name: String,
    is_assigned: bool,
}

#[derive(Clone, Deserialize)]
struct ScoutManager {
    robot_queue: Vec<String>,
    match_key: String,
    scout_map: BiMap<Sid, String>, // socket_id - client_scout.name
}

#[allow(unused)]
impl ScoutManager {
    fn new() -> Self {
        ScoutManager {
            robot_queue: vec![],
            match_key: String::from(""),
            scout_map: BiMap::new(),
        }
    }

    fn get_next_robot(&mut self) -> Option<String> {
        self.robot_queue.pop()
    }

    async fn free_scout(io: SocketIo, client_id: Sid) {
        let mut assigned_scouts = match io.to("assigned_scouts").sockets() {
            Ok(scouts) => scouts,
            Err(err) => panic!("No assigned scouts avaliable"),
        };
        assigned_scouts.iter_mut().for_each(|socket| {
            if socket.id == client_id {
                socket.leave("assigned_scouts");
            }
        });
    }

    /// Returns as many scout-team pairs as possible, pushes the remaining teams to the queue
    fn create_match(
        &mut self,
        mut pending_scouts: Vec<Sid>,
        red_robots: Vec<String>,
        blue_robots: Vec<String>,
    ) -> Vec<(Sid, String)> {
        let mut ret: Vec<(Sid, String)> = vec![];

        self.assign_robots(red_robots, &mut pending_scouts, &mut ret);
        self.assign_robots(blue_robots, &mut pending_scouts, &mut ret);

        ret
    }

    /// Helper for create_match()
    /// Not designed as part of the api
    fn assign_robots(
        &mut self,
        robots: Vec<String>,
        scouts: &mut Vec<Sid>,
        ret: &mut Vec<(Sid, String)>,
    ) {
        robots.iter().for_each(|team| {
            let scout = scouts.pop();
            match scout {
                Some(client_id) => ret.push((client_id, team.to_string())),
                None => self.robot_queue.push(team.to_string()),
            };
        });
    }
}

pub fn on_connect(socket: SocketRef) {
    info!("Socket Connection: {}", socket.id);

    socket.on(
        "scout_req_team",
        move |socket: SocketRef,
              Data::<Scout>(client_scout),
              manager: State<Mutex<ScoutManager>>| {
            let manager = &mut manager.lock().unwrap();

            let socket_rooms = socket
                .rooms()
                .unwrap()
                .into_iter()
                .map(|room| room.to_string())
                .collect::<Vec<String>>();
            let is_assigned = socket_rooms.contains(&String::from("assigned_scouts"));
            let is_pending = socket_rooms.contains(&String::from("pending_scouts"));

            if client_scout.is_assigned || is_assigned {
                error!("Assigned scout requested match");
            } else if is_pending {
                error!("Pending scout requested match");
            }

            info!("Scout Name: {}", client_scout.name);

            manager.scout_map.remove_by_right(&client_scout.name);
            manager
                .scout_map
                .insert(socket.id, client_scout.name.clone());

            socket
                .join("pending_scouts")
                .expect("To be able to join a room");

            info!("Added {} to the pending_scouts room", client_scout.name);
            match manager.get_next_robot() {
                Some(robot) => {
                    info!("Robot avaliable");
                    info!("Removed {} from the pending_scouts room", client_scout.name);
                    info!("Match Key: {}", manager.match_key);

                    let scout_ret = ScoutReturn {
                        team_key: robot.clone(),
                        match_key: manager.match_key.clone(),
                    };
                    let admin_ret = AdminReturn {
                        team_key: robot,
                        name: client_scout.name.to_string(),
                    };

                    socket
                        .leave("pending_scouts")
                        .expect("To be able to leave a room");
                    socket
                        .join("assigned_scouts")
                        .expect("To be able to join a room");
                    match socket.emit("assign_team", scout_ret) {
                        Ok(_) => info!("Assignment Sent Back to Scout"),
                        Err(err) => {
                            error!("Failed to Send Assignment Back to Scout");
                            panic!("{}", err)
                        }
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
        },
    );

    socket.on(
        "admin_create_match",
        move |socket: SocketRef, Data::<Match>(robots), manager: State<Mutex<ScoutManager>>| {
            let manager = &mut manager.lock().unwrap();

            let pending_scouts = match socket.within("pending_scouts").sockets() {
                Ok(scouts) => scouts
                    .into_iter()
                    .map(|socket| socket.id)
                    .collect::<Vec<Sid>>(),
                Err(_err) => {
                    info!("Match Create With No Avaliable Scouts");
                    vec![]
                }
            };
            let scout_matches =
                manager.create_match(pending_scouts, robots.red_robots, robots.blue_robots);

            info!("Admin Create MatchKey: {}", robots.match_key);
            info!("Match being created");
            info!("Number of scout_matches: {}", scout_matches.len());

            scout_matches.into_iter().for_each(|(id, team)| {
                info!("Team: {}", team);
                info!("Scout Socket Id: {}", id);
                info!("\nAdmin Match Key: {}", robots.match_key);

                let name = manager
                    .scout_map
                    .get_by_left(&id)
                    .expect("Scout to be in map");

                let scout_ret = ScoutReturn {
                    team_key: team.clone(),
                    match_key: (*manager).match_key.clone(),
                };
                let admin_ret = AdminReturn {
                    team_key: team,
                    name: name.to_string(),
                };

                let scout_socket = socket
                    .broadcast()
                    .get_socket(id)
                    .expect("Socket In Map To Exist");
                scout_socket
                    .leave("pending_scouts")
                    .expect("To be able to leave a room");
                scout_socket
                    .join("assigned_scouts")
                    .expect("To be able to leave a room");

                match scout_socket.emit("assign_team", scout_ret) {
                    Ok(()) => info!("Sent Team Assignment to Scout"),
                    Err(err) => {
                        error!("Team Assignment Emission Failed\nError: {}", err);
                    }
                };

                match socket.emit("team_match_assigned_admin", admin_ret) {
                    Ok(()) => info!("Team Match Assignment Sent Back to Admin"),
                    Err(err) => {
                        error!(
                            "Team Match Assignment Failed to Send Back to Admin\nError: {}",
                            err
                        );
                    }
                };
            });
        },
    );

    socket.on(
        "scout_submit_match",
        move |socket: SocketRef,
              Data::<TeamMatchData>(submission),
              manager: State<Mutex<ScoutManager>>| {
            let manager = manager.lock().unwrap();
            info!(
                "Data Submitted: {:?} by scout {}",
                submission,
                manager
                    .scout_map
                    .get_by_left(&socket.id)
                    .expect("Scout to be assigned")
            );
            socket
                .leave("assigned_scouts")
                .expect("To be able to leave a room");
        },
    );

    socket.on_disconnect(|socket: SocketRef, reason: DisconnectReason| {
        info!("Socket {} disconnected. Because {}", socket.id, reason);
    })
}

pub fn create_layer() -> (SocketIoLayer, SocketIo) {
    SocketIo::builder()
        .with_state(Mutex::new(ScoutManager::new()))
        .build_layer()
}

