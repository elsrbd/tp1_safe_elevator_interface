use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State
 {
    Idle,
    MovingUp,
    MovingDown,
    DoorsOpen,
 }

 #[derive(Debug, Clone, Copy, PartialEq, Eq)]
 pub enum ElevatorError
 {
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
    queue: VecDeque<u32>, 
}

 #[derive(Debug, Clone, PartialEq, Eq)]
pub struct Status {
    pub floor: u32,
    pub state: State,
    pub queue: VecDeque<u32>,
}

impl Status {
    pub fn new(elevator: Elevator) -> Self {
        Self {floor: elevator.floor(), state: elevator.state(), queue: elevator.queue().clone()}
    }
}

impl Elevator {
    pub fn new(floor_init: u32) -> Result<Self, ElevatorError> {
        if floor_init > 5 {
            return Err(ElevatorError::InvalidFloor(floor_init))
        }
        Ok(Self {floor: floor_init, state: State::Idle, queue: VecDeque::from([])})
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn floor(&self) -> u32 {
        self.floor
    }

    pub fn queue(&self) -> &VecDeque<u32> {
        &self.queue
    }

    pub fn call(&mut self, new_floor: u32) -> Result<&'static str, ElevatorError> {
        match new_floor {
            val if val == self.floor => return Err(ElevatorError::InvalidFloor(new_floor)),
            val if val > 5 => return Err(ElevatorError::InvalidFloor(new_floor)),
            val if val < self.floor => { 
                self.state = State::MovingDown;
                self.queue.push_back(new_floor);
                Ok("call accepted")
            },
            _ => { self.state = State::MovingUp;
                self.queue.push_back(new_floor);
                Ok("call accepted")}
        }
    }

    pub fn step(&mut self) -> Result<&'static str, ElevatorError> {
        match self.state {
            State::MovingDown => {self.floor = self.floor - 1;
                                    if self.queue[0]== self.floor {
                                        self.state = State::DoorsOpen;
                                        self.queue.remove(0); }
                                    Ok("Moving down")},
            State::MovingUp => {self.floor = self.floor + 1;
                                    if self.queue[0]== self.floor {
                                        self.state = State::DoorsOpen;
                                        self.queue.remove(0); }
                                    Ok("Moving up")},
            State::Idle => return Err(ElevatorError::EmptyQueue),
            State::DoorsOpen => return Err(ElevatorError::CannotMoveDoorsOpen),
        }
    }

    pub fn open_doors(&mut self) -> Result<&'static str, ElevatorError> {
        match self.state {
            State::DoorsOpen => return Err(ElevatorError::DoorsAlreadyOpen),
            State::Idle => {self.state = State::DoorsOpen;
                Ok("Doors opening")},
            _ => return Err(ElevatorError::CannotOpenWhileMoving),
        }
        
    }

    pub fn close_doors(&mut self) -> Result<&'static str, ElevatorError> {
        match self.state {
            State::DoorsOpen => {self.state = State::Idle;
                Ok("Doors closing")},
            _ => return Err(ElevatorError::DoorsAlreadyClosed),            
        }
        
    }

    pub fn status(self) -> Status {
        Status::new(self)
    }
}
