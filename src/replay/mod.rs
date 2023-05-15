use std::collections::HashMap;

mod event;
pub(super) use event::*;

mod ui;
pub use ui::*;

pub struct Replay {
    ticks: Vec<ReplayState>,

    pub time: f32,
    pub end: f32,
}

impl Replay {
    pub fn from_str(data: &str) -> anyhow::Result<Self> {
        let events: Vec<RawEvent> = serde_json::from_str(data)?;
        let mut end = 0f32;
        for event in &events {
            end = end.max(event.time);
        }

        // Split events per "tick"
        let mut events_per_tick: Vec<Vec<RawEvent>> = Vec::new();
        let mut tick_events: Vec<RawEvent> = Vec::new();
        let mut cur_tick = 0;
        for event in &events {
            let tick = (event.time * 25.0) as usize;
            if tick > cur_tick {
                events_per_tick.push(tick_events);
                tick_events = Vec::new();
                cur_tick += 1;
            }

            tick_events.push(event.clone());
        }

        let mut ticks: Vec<ReplayState> = Vec::new();
        let mut cur_state = ReplayState {
            players: HashMap::new(),
            cars: HashMap::new(),
        };

        for tick_events in events_per_tick {
            for event in tick_events {
                cur_state.process_event(event);
            }
            ticks.push(cur_state.clone());
        }

        Ok(Self {
            ticks,

            time: 0f32,
            end,
        })
    }

    pub fn get_state(&self) -> ReplayState {
        let mut closest_tick_pre = 0;
        for (tick, state) in self.ticks.iter().enumerate() {
            let time = tick as f32 / 25.0;
            if time < self.time {
                closest_tick_pre = tick;
            } else {
                break;
            }
        }
        self.ticks[closest_tick_pre].clone()
    }
}

#[derive(Clone, Debug)]
pub struct Player {
    pub id: usize,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Car {
    pub pid: usize,
    pub vid: usize,

    pub pos: (f32, f32, f32),
    pub rot: (f32, f32, f32, f32),
    pub vel: (f32, f32, f32),
    pub rvel: (f32, f32, f32),

    pub size: (f32, f32),
}

#[derive(Clone, Debug)]
pub struct ReplayState {
    pub players: HashMap<usize, Player>,
    pub cars: HashMap<(usize, usize), Car>,
}

impl ReplayState {
    fn process_event(&mut self, raw_event: RawEvent) {
        if let Some(event) = raw_event.into_event() {
            match event {
                Event::PlayerJoin((pid, event_player_join)) => {
                    self.players.insert(pid, Player {
                        id: pid,
                        name: event_player_join.name.clone(),
                    });
                },
                Event::PlayerLeave(pid) => {
                    self.players.remove(&pid);
                    // When a player leaves, remove all his existing cars, just to make sure
                    let mut vids = Vec::new();
                    for ((search_pid, vid), _car) in &self.cars {
                        if pid == *search_pid { vids.push(*vid); }
                    }
                    for vid in vids {
                        self.cars.remove(&(pid, vid));
                    }
                },

                Event::VehicleSpawn((pid, vid, _data)) => {
                    let car = Car {
                        pid,
                        vid,

                        pos: (0f32, 0f32, 0f32),
                        rot: (0f32, 0f32, 0f32, 0f32),
                        vel: (0f32, 0f32, 0f32),
                        rvel: (0f32, 0f32, 0f32),

                        size: (0f32, 0f32),
                    };
                    self.cars.insert((pid, vid), car);
                },
                Event::VehicleDelete((pid, vid)) => {
                    self.cars.remove(&(pid, vid));
                },
                Event::VehiclePosition((pid, vid, data)) => {
                    if let Some(car) = self.cars.get_mut(&(pid, vid)) {
                        car.pos = (data.pos[0], data.pos[1], data.pos[2]);
                        car.rot = (data.rot[0], data.rot[1], data.rot[2], data.rot[3]);
                        car.vel = (data.vel[0], data.vel[1], data.vel[2]);
                        car.rvel = (data.rvel[0], data.rvel[1], data.rvel[2]);
                    }
                },
                Event::VehicleSize((pid, vid, data)) => {
                    // We don't have to store the event for later, it should be impossible to receive this event
                    // before the spawn event. The vehicle spawn handler first saves its own event before
                    // requesting vehicle size and waiting for a response.
                    if let Some(car) = self.cars.get_mut(&(pid, vid)) {
                        car.size = (data.size[0], data.size[1]);
                    }
                },

                _ => {},
            }
        }
    }
}
