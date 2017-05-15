# game
Hot reload game playing.

Arrows to move, Z to jump.

Oly tested on Linux for now but no reason it shouldn't work on anything else.

To use:
 - Create an "Engine" inside the `engine` sub-project. This creates a window and provides hooks for the editor. I have made an SDL one for now.
 - Create your game inside the `game` sub-project. Ideally run `cargo watch -x build` on this project.
 - Call `cargo run` on the `runner`. See shortcuts for help.
 - Alternatively, `cargo build` on `redist` to create a redistributable that runs your game without a runner.

Shortcuts
 - Ctrl+W - Actively watches for the game library to change and updates to use that library automatically.
 - Ctrl+C - Toggles enabling compatibility check; incompatible builds will not be restarted automatically if watching.
 - Ctrl+R - Restarts the game from the beginning.
 - Ctrl+P - Pauses the game.
 - Ctrl+S - Print the runner status (e.g. whether watching).
