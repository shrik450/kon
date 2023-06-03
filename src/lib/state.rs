pub(super) enum State {
    Todo,
    Next,
    Doing,
    Hold,
    Cancelled,
    Done,
}

pub(super) enum StateGroup {
    Todo,
    InProgress,
    Completed,
}

impl State {
    pub(super) fn next(&self) -> Option<State> {
        match self {
            State::Todo => Some(State::Next),
            State::Next => Some(State::Doing),
            State::Doing => Some(State::Done),
            State::Hold => Some(State::Doing),
            State::Cancelled => None,
            State::Done => None,
        }
    }
}

impl TryFrom<&str> for State {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "todo" | "TODO" => Ok(State::Todo),
            "next" | "NEXT" => Ok(State::Next),
            "doing" | "DOING" => Ok(State::Doing),
            "hold" | "HOLD" => Ok(State::Hold),
            "cancelled" | "CANCELLED" => Ok(State::Cancelled),
            "done" | "DONE" => Ok(State::Done),
            _ => Err(format!("Invalid text for state: {:?}", s)),
        }
    }
}

impl Into<StateGroup> for State {
    fn into(self) -> StateGroup {
        match self {
            State::Todo => StateGroup::Todo,
            State::Next => StateGroup::Todo,
            State::Doing => StateGroup::InProgress,
            State::Hold => StateGroup::InProgress,
            State::Cancelled => StateGroup::Completed,
            State::Done => StateGroup::Completed,
        }
    }
}

impl Into<&str> for State {
    fn into(self) -> &'static str {
        match self {
            State::Todo => "TODO",
            State::Next => "NEXT",
            State::Doing => "DOING",
            State::Hold => "HOLD",
            State::Cancelled => "CANCELLED",
            State::Done => "DONE",
        }
    }
}

impl Into<&str> for StateGroup {
    fn into(self) -> &'static str {
        match self {
            StateGroup::Todo => "Todo",
            StateGroup::InProgress => "In Progress",
            StateGroup::Completed => "Completed",
        }
    }
}
