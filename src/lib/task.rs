use super::state::State;

pub(super) struct Task {
    id: usize,
    state: State,
    title: String,
    assign: Option<String>,
    priority: Option<String>,
    tags: Vec<String>,
}

enum ParsedValue<T> {
    Unassigned,
    Present(T),
}

impl<T> ParsedValue<T> {
    fn unwrap_or(self, default: T) -> T {
        match self {
            ParsedValue::Unassigned => default,
            ParsedValue::Present(value) => value,
        }
    }

    fn map_with_optional(self, value: Option<T>) -> Option<T> {
        match self {
            ParsedValue::Unassigned => None,
            ParsedValue::Present(actual_value) => Some(actual_value),
        }
    }
}

pub(super) struct ParsedTask {
    id: ParsedValue<usize>,
    state: ParsedValue<State>,
    title: ParsedValue<String>,
    assign: ParsedValue<String>,
    priority: ParsedValue<String>,
    tags: Vec<String>,
}

pub(super) struct TaskBuilder {
    parsed_task: ParsedTask,
    default_id: usize,
    default_title: String,
    default_state: State,
    default_assign: Option<String>,
    default_priority: Option<String>,
    default_tags: Vec<String>,
}

impl TaskBuilder {
    pub fn new(parsed_task: ParsedTask) -> Self {
        Self {
            parsed_task,
            default_id: 0,
            default_title: String::new(),
            default_state: State::Todo,
            default_assign: None,
            default_priority: None,
            default_tags: Vec::new(),
        }
    }

    pub fn id(self, id: usize) -> Self {
        TaskBuilder {
            default_id: id,
            ..self
        }
    }

    pub fn title(self, title: String) -> Self {
        TaskBuilder {
            default_title: title,
            ..self
        }
    }

    pub fn state(self, state: State) -> Self {
        TaskBuilder {
            default_state: state,
            ..self
        }
    }

    pub fn assign(self, assign: Option<String>) -> Self {
        TaskBuilder {
            default_assign: assign,
            ..self
        }
    }

    pub fn priority(self, priority: Option<String>) -> Self {
        TaskBuilder {
            default_priority: priority,
            ..self
        }
    }

    pub fn build(self) -> Task {
        Task {
            id: self.parsed_task.id.unwrap_or(self.default_id),
            state: self.parsed_task.state.unwrap_or(self.default_state),
            title: self.parsed_task.title.unwrap_or(self.default_title),
            assign: self
                .parsed_task
                .assign
                .map_with_optional(self.default_assign),
            priority: self
                .parsed_task
                .priority
                .map_with_optional(self.default_priority),
            tags: self.parsed_task.tags,
        }
    }
}
