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
other people to be able to interact with.

In contrast, kon uses markdown, which most people are familiar with. Its
extensions to GFM tasks help you squeeze a little extra juice from text-based
tasks. Kon is gracious and will accept - even fix! - files that don't strictly
follow its spec, and its tasks degrade gracefully even if you're editing it by
hand without kon around.

## Key Features

- **Command-Line Interface:** Kon lives in your terminal, allowing you to manage
  tasks quickly and efficiently without context switching.
- **Markdown Format:** Tasks are stored in a simple markdown format that is easy
  to read and edit manually.
- **Task States:** Easily mark tasks as `todo`, `next`, `in-progress`, `on
hold`, `cancelled`, or `done`.
- **Task Priorities:** Assign priorities to tasks as `high`, `med`, or `low`.
- **Assignees:** Assign tasks to team members directly in the markdown file.
- **Tags:** Organize your tasks with custom tags.
- **Task Dependencies:** Define and track task dependencies.
- **Task Notes:** Add detailed notes and descriptions to your tasks.
- **Dashboard:** Like org agenda, shows you what you're working on so you can
  tuck it away in some corner of your screen, and pick up where you left off the
  next time you come back to your weekend project.

## Task Format

Here's an example of how tasks are represented in Kon:

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

For more details, you can read the [Kon Markdown Specification](doc/task_spec.md). 
You can also read Kon's own Kon Todos at [todo.md](todo.md)

## Getting Started

TBD.

## Usage

See [usage.md](doc/usage.md)
