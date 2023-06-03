# Kon Markdown Specification

## Overview

Kon uses a simple and straightforward markdown format to store tasks. This
document outlines the standard format for representing tasks.

## Task Representation

Each task is represented as a markdown list item with a checkbox. The task
description, state, priority, assignee, due date, and tags are all included
inline.

Here is a sample task representation:

```markdown
- [ ] (state: todo, priority: \*\*\*) #1 Implement login functionality @john (Due: 31-12-2023) #backend #authentication
```

### Task States

Each task has a state which is represented in brackets `(state: state_name)`.
The state must be one of the following:

- `todo`: The task has been created but work has not yet started.
- `next`: The task is the next task to work on once the current task is completed.
- `in-progress`: Work has started on the task but it's not yet completed.
- `hold`: The task is currently on hold and work will resume later.
- `cancelled`: The task has been cancelled and will not be completed.
- `done`: The task has been completed.

### Task Priority

Each task has a priority which is represented in brackets `(priority:
priority_level)`. The priority level must be one of the following:

- `High`: High priority.
- `Med`: Medium priority.
- `Low`: Low priority.

### Task Number

Each task is assigned a unique number which is represented as `#task_number`.
The task number is used to refer to the task in the system.

### Assignee

The person assigned to the task is represented using the `@` symbol followed by
their username. For example, `@john`.

### Tags

Tags can be added to the task using hashtags. For example, `#backend
#authentication`.

## Task Notes

Notes can be added below the task in a paragraph.

```markdown
- [ ] (state: todo, priority: \*\*\*) #1 Implement login functionality @john (Due: 31-12-2023) #backend #authentication
  - Note: Remember to add validation for inputs.
```

## Task Dependencies

Dependencies can be represented by linking the dependent task number. For
example, `(Depends on #2)`.

## Task Sections

Tasks are grouped into sections based on their state. The sections are:

- Todo: Includes tasks with state `todo` and `next`.
- In Progress: Includes tasks with state `in-progress` and `hold`.
- Completed: Includes tasks with state `done` and `cancelled`.

Here's an example:

```markdown
# Todo

- [ ] (state: todo, priority: \*\*\*) #1 Implement login functionality @john (Due: 31-12-2023) #backend #authentication
- [ ] (state: next, priority: \*\*) #2 Write unit tests for login functionality @mary (Depends on #1) #testing

# On Hold

- [ ] (state: in-progress, priority: \*) #3 Set up database @alex #database
- [ ] (state: hold, priority: \*) #6 Research new technologies @john

# Completed

- [x] (state: cancelled, priority: \*\*\*) #5 Implement sign up functionality @mary
- [x] (state: done, priority: \*) #4 Set up project repository @john
```
