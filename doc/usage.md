# KON(1) - Kon Command-Line Task Manager

## SYNOPSIS

`kon <command> [options]`

## DESCRIPTION

Kon is a command-line task manager that utilizes a `todo.md` file in the root
directory of your project.

## COMMANDS

- `init`: Initializes a new `todo.md` file in the current directory.
- `add` (aliased to `a`): Adds a new task.
- `edit <task number>` (aliased to `e`): Edits an existing task.
- `toggle <task number>` (aliased to `t`): Set the status of a task to the next
  state.
- `ls`: Lists all tasks.
- `dashboard` (aliased to `db`): Shows a list of tasks assigned to you.
- `find <query>` (aliased to `fd`): Search through tasks by text.
- `filter` (aliased to `f`): Filter tasks.
- `help`: Displays help information about kon.

## OPTIONS

You can use these options when editing or filtering.

- `-s, --state <state>`
- `-p, --priority <priority>`
- `-n, --note <note>` (Only useful when editing - ignored for filter. Use `find` instead.)
- `-g, --tag <tag>`
- `-d, --depend <task_number>`
- `-a, --assign <user>`

When filtering, you can use `*` as a wildcard to filter for presence of any
value.

## EXAMPLES

Initialize a new `todo.md` file:

```bash
$ kon init
```

Add a new task:

```bash
$ kon a "Implement login functionality"
```

Change the state of a task:

```bash
$ kon e 1 -s next
```

Assign a task to a user:

```bash
$ kon e 1 -a john
```

If you want to make a lot of changes to a task, or edit it in detail, you can:

```bash
$ kon e 1
```

which will bring it up in your `$EDITOR`.

List all tasks:

```bash
$ kon ls
```

Show the dashboard:

```bash
$ kon db
```

Find tasks which have "login" anywhere in the text:

```bash
$ kon fd login
```

Filter tasks on hold which depend on task 1:

```bash
$ kon f --state hold --depend 1
```

## AUTHOR

Written by Shrikanth Upadhayaya.
