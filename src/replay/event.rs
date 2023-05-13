#[derive(Deserialize, Clone)]
pub struct RawEvent {
    pub kind: String,
    pub time: f32,
    pub player_id: Option<f32>,
    pub vehicle_id: Option<f32>,
    pub data: Option<serde_json::Value>,
}

impl RawEvent {
    pub fn into_event(self) -> Option<Event> {
        match self.kind.as_str() {
            "EVENT_PLAYER_JOIN" => {
                if let Some(data) = self.data {
                    if let Ok(event_player_join) = serde_json::from_value(data) {
                        if let Some(pid) = self.player_id {
                            Some(Event::PlayerJoin((pid as usize, event_player_join)))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            "EVENT_PLAYER_LEAVE" => {
                if let Some(pid) = self.player_id {
                    Some(Event::PlayerLeave(pid as usize))
                } else {
                    None
                }
            },
            "EVENT_VEH_SPAWN" => {
                if let Some(data) = self.data {
                    if let Ok(decoded) = serde_json::from_value(data) {
                        if let Some(pid) = self.player_id {
                            if let Some(vid) = self.vehicle_id {
                                Some(Event::VehicleSpawn((pid as usize, vid as usize, decoded)))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            "EVENT_VEH_DELETE" => {
                if let Some(pid) = self.player_id {
                    if let Some(vid) = self.vehicle_id {
                        Some(Event::VehicleDelete((pid as usize, vid as usize)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            "EVENT_VEH_POS" => {
                if let Some(data) = self.data {
                    if let Ok(decoded) = serde_json::from_value(data) {
                        if let Some(pid) = self.player_id {
                            if let Some(vid) = self.vehicle_id {
                                Some(Event::VehiclePosition((pid as usize, vid as usize, decoded)))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            "EVENT_VEH_SIZE" => {
                if let Some(data) = self.data {
                    if let Ok(decoded) = serde_json::from_value(data) {
                        if let Some(pid) = self.player_id {
                            if let Some(vid) = self.vehicle_id {
                                Some(Event::VehicleSize((pid as usize, vid as usize, decoded)))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            _ => None
        }
    }
}

pub enum Event {
    Unsupported,

    PlayerJoin((usize, EventPlayerJoin)),
    PlayerLeave(usize),

    VehicleSpawn((usize, usize, EventVehicleSpawn)),
    VehicleDelete((usize, usize)),

    VehiclePosition((usize, usize, EventVehiclePosition)),
    VehicleSize((usize, usize, EventVehicleSize)),
}

#[derive(Deserialize)]
pub struct EventPlayerJoin {
    pub name: String,
}

#[derive(Deserialize)]
pub struct EventVehicleSpawn(serde_json::Value); // TODO: Parse vehicle data?

#[derive(Deserialize)]
pub struct EventVehiclePosition {
    pub ping: f32,
    pub pos: [f32; 3],
    pub rot: [f32; 4],
    pub vel: [f32; 3],
    pub rvel: [f32; 3],
}

#[derive(Deserialize)]
pub struct EventVehicleSize {
    pub size: [f32; 2],
}
