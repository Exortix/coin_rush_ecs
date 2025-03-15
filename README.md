
---

# **Rust ECS Game Documentation (Work in progress)**

## **Overview**
A simple 2D game using SDL2 and Specs ECS framework. Features player movement, entity spawning, and basic rendering.

## **Components**
| Component          | Type/Storage       | Description                          |
|--------------------|--------------------|--------------------------------------|
| Position           | VecStorage         | x/y coordinates (f32)                |
| Velocity           | VecStorage         | dx/dy movement speed (f32)           |
| Health             | VecStorage         | Health value (u32)                   |
| Score              | VecStorage         | Score value (u32)                    |
| PlayerControlled   | NullStorage        | Marker for player entity             |
| Coin               | NullStorage        | Marker for collectible coins         |
| Enemy              | NullStorage        | Marker for enemy entities            |
| Obstacle           | NullStorage        | Marker for obstacles (unused)        |
| PowerUp            | VecStorage         | Contains power_type (String)         |

## **Systems**
### **RenderSystem**
- **Function**: Draws all entities to the screen
- **Canvas**: Uses SDL2 canvas for rendering
- **Rendering Logic**:
  - Players: Green 20x20 squares
  - Coins: Yellow 10x10 squares
  - Enemies: Red 20x20 squares
- **Execution**: Runs last in the game loop

### **PlayerInputSystem**
- **Input Handling**: 
  - Reads keyboard input from `InputResource`
  - Updates player velocity based on WASD keys
  - Normalizes diagonal movement (dx/dy ∈ {-1, 0, 1})

### **PhysicsSystem**
- **Movement**: 
  - Updates positions based on velocity
  - Scales movement by 5.0 units/frame (frame-rate dependent)

### **SpawnSystem**
- **Timers**: Spawns entities at intervals:
  - Coins: Every 5 seconds
  - Enemies: Every 10 seconds
  - Power-ups: Every 15 seconds
- **Randomization**: Uses thread_rng() for positions
- **Power-up Types**: Randomly chooses "health" or "speed"

## **Game Loop**
1. **Event Handling**: Processes quit/escape events
2. **Input Update**: Captures keyboard state
3. **System Execution Order**:
   1. PlayerInputSystem
   2. PhysicsSystem
   3. SpawnSystem
   4. RenderSystem
4. **Frame Rate**: Fixed 60 FPS (16.6ms frame sleep)

## **Entities**
### **Player**
- Components: Position, Velocity, PlayerControlled, Health(100), Score(0)
- Start Position: (400, 300)

### **Initial Coin**
- Position: (200, 200)

### **Initial Enemy**
- Position: (600, 100)
- Health: 30
- Velocity: (0, 0) (static)

## **Known Limitations**
1. **Missing Systems**:
   - Collision detection
   - Combat/damage system
   - Power-up effects
   - Enemy AI/movement
2. **Frame-rate Dependent**:
   - Movement speed (needs delta time)
   - Spawn timers (use delta time accumulation)
3. **Unimplemented Features**:
   - Obstacle interactions
   - Score updates
   - Health regeneration
4. **Potential Issues**:
   - Spawned enemies have no velocity
   - No bounds checking for entity positions
   - Power-up types use String (better as enum)

## **Dependencies**
- sdl2 = "0.35.2"
- specs = "0.16.1"
- specs-derive = "0.4.1"
- rand = "0.8.5"

## **Usage**
1. Build with `cargo build --release`
2. Run executable to start game
3. Controls: WASD to move, Esc to quit
