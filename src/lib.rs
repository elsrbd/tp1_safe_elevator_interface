#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Idle,
    MovingUp,
    MovingDown,
    DoorsOpen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElevatorError {
    InvalidFloor(u32),
    DoorsAlreadyOpen,
    DoorsAlreadyClosed,
    CannotOpenWhileMoving,
    CannotMoveDoorsOpen,
    EmptyQueue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Elevator {
    floor: u32,
    state: State,
    queue: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Status {
    pub floor: u32,
    pub state: State,
    pub queue: Vec<u32>,
}

impl Elevator {
    pub fn new(floor_init: u32) -> Result<Self, ElevatorError> {
        if floor_init > 5 {
            return Err(ElevatorError::InvalidFloor(floor_init));
        }
        Ok(Self {
            floor: floor_init,
            state: State::Idle,
            queue: Vec::new(),
        })
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn floor(&self) -> u32 {
        self.floor
    }

    pub fn queue(&self) -> &[u32] {
        &self.queue
    }

    pub fn call(&mut self, new_floor: u32) -> Result<(), ElevatorError> {
        match new_floor {
            val if val == self.floor || self.queue.contains(&val) => Ok(()),
            val if val > 5 => Err(ElevatorError::InvalidFloor(new_floor)),
            _ => {
                self.queue.push(new_floor);

                if self.state == State::Idle {
                    self.update_direction();
                }

                Ok(())
            }
        }
    }

    pub fn step(&mut self) -> Result<(), ElevatorError> {
        if self.state == State::DoorsOpen {
            return Err(ElevatorError::CannotMoveDoorsOpen);
        }

        if self.queue.is_empty() {
            self.state = State::Idle;
            return Err(ElevatorError::EmptyQueue);
        }

        let target = self.queue[0];
        if target > self.floor {
            self.floor += 1;
        } else if target < self.floor {
            self.floor -= 1;
        }

        if self.floor == target {
            self.queue.remove(0);
            self.state = State::DoorsOpen;
        } else {
            self.update_direction();
        }

        Ok(())
    }

    pub fn open_doors(&mut self) -> Result<(), ElevatorError> {
        match self.state {
            State::DoorsOpen => Err(ElevatorError::DoorsAlreadyOpen),
            State::Idle => {
                self.state = State::DoorsOpen;
                Ok(())
            }
            _ => Err(ElevatorError::CannotOpenWhileMoving),
        }
    }

    pub fn close_doors(&mut self) -> Result<(), ElevatorError> {
        match self.state {
            State::DoorsOpen => {
                if self.queue.is_empty() {
                    self.state = State::Idle;
                } else {
                    self.update_direction();
                }
                Ok(())
            }

            _ => Err(ElevatorError::DoorsAlreadyClosed),
        }
    }

    pub fn status(&self) -> Status {
        Status {
            floor: self.floor,
            state: self.state,
            queue: self.queue.clone(),
        }
    }

    fn update_direction(&mut self) {
        if let Some(&target) = self.queue.first() {
            if target > self.floor {
                self.state = State::MovingUp;
            } else if target < self.floor {
                self.state = State::MovingDown;
            }
        } else {
            self.state = State::Idle;
        }
    }
}
