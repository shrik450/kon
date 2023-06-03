# Kon Markdown Specification

## Overview

Kon uses a simple and straightforward markdown format to store tasks. This
document outlines the standard format for representing tasks.

## Task Representation

Each task is represented as a markdown list item with a checkbox. The task
description, state, priority, assignee and tags are all included
inline.

Here is a sample task representation:

```markdown
- [ ] TODO #1 Implement login functionality @john {high} #backend #authentication
```

### Task States

Each task has a state immediately following the checbox. The state is a single
word in all caps. It must be one of the following:

- `TODO`
- `NEXT`
- `DOING`
- `HOLD`
- `CANCELLED`
- `DONE`

### Task Priority

Each task has a priority which is represented in braces, like `{
priority_level}`. The priority level must be one of the following:

- `high`: High priority.
- `med`: Medium priority.
- `low`: Low priority.

### Task Number

Each task is assigned a unique number which is represented as `#task_number`. If
you're editing a kon task file by hand, you can use `#?` and run `kon fix` to
auto assign.

### Assignee

If you're working with multiple people, you can assign tasks to people using `@`
followed by their handle. Kon doesn't care what the handles are - just that you
and the people you're working with agree on them. For example, `@john`.

### Tags

Tags can be added to the task using hashtags. For example, `#backend
#authentication`.

## Task Notes

Notes can be added below the task in a paragraph.

```markdown
- [ ] (state: todo) (priority: high) #1 Implement login functionality @john (Due: 31-12-2023) #backend #authentication
      Remember to add validation for inputs!
```

## Task Dependencies

Dependencies can be represented by linking the dependent task number. For
example, `(Depends on #2)`. You probably shouldn't be editing this by hand - use
the kon command instead.

## Task Sections

Tasks are grouped into sections based on their state. The sections are:

- Todo: Includes tasks with state `todo` and `next`.
- In Progress: Includes tasks with state `in-progress` and `hold`.
- Completed: Includes tasks with state `done` and `cancelled`.

Once again, if you're editing tasks by hand, you can fall back to `kon fix` to
move tasks around based on their states.

## Subtasks

Subtasks work exactly as they do with regular markdown: you add a task list to
the description. You can then work with them with `kon e`.

# Big Notes

If you have big tasks that require a lot of detailed notes, subtasks etc., you
can spread them out into a dedicated file in a `.kon`folder. Kon will
automatically link a task to its file in the folder by numner, so e.g. `#1` will
go to `.kon/1.md`. `kon edit` will automatically pick up the right place. For
redundancy and performance, Kon will still store the header in the tasks.md
file.

For those writing or editing tasks by hand, Kon will link the header to the file
so they know there's more elsewhere.

```markdown
- [ ] [TODO #4 Create a big doc](./.kon4.md) {high} @me
- [ ] TODO #5 Add more users {low}
```

# Bringing Everything Together

Here's an example:

```markdown
# Todo

- [ ] NEXT #5 Implement login functionality {high} @john #backend #authentication
- [ ] TODO #6 Write unit tests for login functionality {low} (Depends on #1) #testing
- [ ] #? Send confirmation e-mail #authentication {med}

# In Progress

- [ ] DOING #4 Set up database {high}

# Completed

- [x] DONE #3 Implement sign up functionality {high}
- [x] DONE #1 Set up project repository @john
- [x] CANCELLED #2 Research new technologies @john {low}
```
