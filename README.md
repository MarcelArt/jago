# Tempa 🔥🔨
## Easily startup Godot game written in Rust

### Installation
`cargo install --git https://github.com/MarcelArt/tempa --branch master`

### Usage
#### Creating new Godot + Rust Project
1. Create new Godot + Rust Project
```sh
tempa "<project name>" 
# or
tempa new "<project name>"

# examples
tempa "Half Life Clone" 
# or
tempa new "Half Life Clone" 
```
2. It will create a new Rust project directory[^1]
```
half_life_clone
 ┣ godot
 ┃ ┣ .editorconfig
 ┃ ┣ icon.svg
 ┃ ┣ gdrust.gdextension
 ┃ ┗ project.godot
 ┣ src
 ┃ ┣ lib.rs
 ┃ ┗ rust_example.rs
 ┣ .gitignore
 ┣ Cargo.lock
 ┗ Cargo.toml
```
3. If you see this in your terminal you are successfully initialize your project
```sh
Project Half Life Clone created successfully!
You can now run the project with:
1. cd half_life_clone
2. cargo build
3. Open the /godot directory using Godot 4.1 or later
4. Run the project
5. Enjoy!
```
4. Next step you can `cd` to the project directory and build the godot rust extension with `cargo build`
5. Then open the `/godot` directory in Godot Engine
6. If the `RustExample` is exist under `Node2D` in Create New Node Window, that mean the setup is finished and you can continue add new nodes in Rust and develop your dream game.

#### Add new custom node
1. Inside the root of Rust project run `tempa add <node name> <base node>`
2. Replace `<node name>` with your desired custom node name in "PascalCase"
3. Replace `<base node>` with what native Godot nodes you want to extends to
4. Example: `tempa add MainPlayer CharacterBody2D`
5. It will creates main_player.rs, filename will be written in "snake_case" 
6. Don't forget to import the module in `lib.rs` file
7. You are done, next step you need to modify the generated file

#### Watch changes (Live Reload)
1. Inside the root of Rust project run `tempa dev` or `tempa watch`
2. Everytime there is a change in src folder tempa will automatically rebuild the Rust Library

### Crate Used
#### Godot
This project uses crate called godot 0.3.5, for more information, please read the documentation at https://godot-rust.github.io/book/

[^1]: Note it will create the directory in "snake_case"

## Attributions
https://ghostpixxells.itch.io/pixel-mart
https://piiixl.itch.io/food
https://ninjikin.itch.io/treasure
https://kenney.nl/assets/emotes-pack