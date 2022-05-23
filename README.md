# laydown

`laydown` is a simple CLI application to help you prepare for your next Daily Standup. No longer shall your name be called on only for you to stare into the abyss while you struggle to remember what you did yesterday.

```
$ laydown

DID:
- Reviewed Adam's pull request
- Wrote tests for new endpoint

DOING:
- Writing a script to automate database backups

BLOCKERS:
- Waiting on admin access to production server

SIDEBARS:
- Discuss re-writing everything in Rust with team
```

### How to Install
```
cargo install laydown
```

### How to Use
```
$ laydown help

Running "laydown" without passing any arguments will display your Standup

Usage: laydown <command> "<item>"

Available commands:
di, did      <item>  Add item to DID section of your Standup
do, doing    <item>  Add item to DOING section of your Standup
bl, blocker  <item>  Add item to BLOCKERS section of your Standup
sb, sidebar  <item>  Add item to SIDEBARS section of your Standup

clear                Remove all items from your Standup
edit <editor>        Directly access data displayed in your Standup.
                     This can be used to edit or delete existing entries.
                     Will use VI by default if no editor is provided.
undo                 Remove last item added to your Standup.

archive              Archive today's Standup. Found in laydown config directory.
config-dir           Print location of laydown config directory.

help, --help         Display this message
```
