# Kon

<p align="center">
  <img src="images/kon_logo.png" width="100" alt="Kon Logo" />
</p>

## About

Kon is an ad hoc, informally-specified, bug-ridden, slow implementation of half
of org-mode's task management system. It's also pretty opinionated.

Kon turns your project's `todo.md` file into a tool for managing tasks and
tracking progress. For a small project for a solo developer or a small team,
that's usually enough.

## Why Kon?

I love emacs, and org-mode. I really do. But I don't use emacs as a code editor,
and keeping it open for just org more doesn't work for me. Apart from that, if I
want to check in the todos for a project, org-mode isn't a format I can expect
other people to be able to interact with. Kon uses markdown, which most people
are familiar with, and its extensions to markdown's task scheme degrade
gracefully even if you're editing it by hand.

## Key Features

- **Command-Line Interface:** Kon lives in your terminal, allowing you to manage tasks quickly and efficiently without context switching.
- **Markdown Format:** Tasks are stored in a simple markdown format that is easy to read and edit manually.
- **Task States:** Easily mark tasks as `todo`, `next`, `in-progress`, `on hold`, `cancelled`, or `done`.
- **Task Priorities:** Assign priorities to tasks as `high`, `med`, or `low`.
- **Assignees:** Assign tasks to team members directly in the markdown file.
- **Tags:** Organize your tasks with custom tags.
- **Task Dependencies:** Define and track task dependencies.
- **Task Notes:** Add detailed notes and descriptions to your tasks.
- **Dashboard:** Like org agenda. It shows you what you're working on so you can
  tuck it away in some corner of your screen, and pick up where you left off the
  next time you come back to your weekend project.

## Task Format

Here's an example of how tasks are represented in Kon:

```markdown
# Todo

- [ ] (state: todo) (priority: high) #1 Implement login functionality @john #backend #authentication
  Remember to add validation for inputs!
- [ ] (state: next) (priority: med) #2 Write unit tests for login functionality @mary (Depends on #1) #testing
- [ ] (state: in-progress) (priority: low) #3 Set up database @alex #database

# On Hold

- [ ] (state: hold) (priority: low) #6 Research new technologies @john

# Completed

- [x] (state: cancelled) (priority: high) #5 Implement sign up functionality @mary
- [x] (state: done) (priority: low) #4 Set up project repository @john
```

For more details, you can read the [Kon Markdown Specification](doc/task_spec.md). 
You can also read Kon's own Kon Todos at [todo.md](todo.md)

## Getting Started

TBD.

## Usage

See [usage.md](doc/usage.md)
