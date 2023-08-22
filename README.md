# gluttonous-snake

Rust language version of Gluttonous Snake. Developed by bevy engine.

## How to play

- Just run `cargo run`.
- If you want to build a wasm target and play this game on the web, you can run `zx wasm.mjs` and then open `http://127.0.0.1:8080` to enjoy it.
- I have deployed a [github page](https://sasakiyori.github.io/gluttonous-snake/) for this, you can directly play with it. But I have not create a workflow yet, so the wasm deploy in this page may be outdated.


## Implementations

### [Main Branch](https://github.com/sasakiyori/gluttonous-snake/tree/main)

Snake stores as pieces, every piece owns a `Direction` enum which will control its translation.

```rust
#[derive(Component)]
pub struct Snake(pub Direction);
```

Since the later spawned sprite has bigger entity id (I am not sure for this but at least it is true in practice), we can treat the `Query<Snake>` is an ordered snake body.  

All snake pieces will move with their own direction in every frame, but only when timer ticked, they will change their direction first and then move. Attention, the speed of every frame, the timer duration and snake size should be a divisible relationship, or else the movement of whole snake will break down.

```rust
// n should be a positive integer
let snake_size = speed * timer_duration * n;

// currently: snake_size = 18.0px, speed = 3.0px, timer_duration = 0.1s, frame = 1/60 s
```

The rule for direction change is:  

- header change the direction by keyboard input
- other pieces use the direction of their front piece

![snake move](doc/m1.png)

#### [Patch For Main Branch](https://github.com/sasakiyori/gluttonous-snake/tree/optimize-snake-body-dislocation)

Here is a patch for main branch, which fix the dislocation problem of snake body. I'm not sure if it is the core cause, so it is merged into `optimize-snake-body-dislocation` branch temporarily.


### [Move By Carry Pieces Branch](https://github.com/sasakiyori/gluttonous-snake/tree/move_by_carry_pieces)

Snake stores as pieces, but there is a struct to manage all pieces and the main direction of snake.

```rust
#[derive(Component)]
pub struct SnakePiece;

#[derive(Component)]
pub struct Snake {
    pub body: LinkedList<Entity>,
    pub direction: Direction,
}
```

The idea of this branch is, when snake totally move a length of snake piece size, we can treat it as move the snake tail piece to the snake head.  
![snake move](doc/m2.png)

The advantage for this idea is that we don't need to control movements in every frame. And obviously the disadvantage is the movement of snake looks not so smoothly, and if we want to improve it, we have to speed snake up so that we can flush the pieces quickly.

## TODO

This repo still have many works to do and bugs to fix.

- [x] If we change directions when snake eat bean, it may cause the separation of snake body (Main Branch).
- [ ] Sometimes the snake body is not in the same line (Main Branch).
- [x] Beans and snake should be spawned inside the window.
- [x] Beans and snake should be spawned with grid alignment.
- [x] Better snake dead check.
- [x] The order of dead check and eat check.
- [x] Game Menu.
- [x] Score Display.
- [x] Audio.
- [ ] Better sprite texture.
- [x] WASM support.
- [x] Github page.
- [ ] Workflow for wasm deployment.

## Assets

Audio assets comes from [kenney.nl](https://www.kenney.nl/assets/impact-sounds).
