use bevy::core::FixedTimestep;
use bevy::prelude::*;
use rand::prelude::*;

pub mod grid;


const WIDTH: f32 = 500.0;
const HEIGHT: f32 = 300.0;

const G_WIDTH: f32 = 100.0;
const G_HEIGHT: f32 = 100.0;

const CELL_SIZE: f32 = 5.0;


const CELL_COLOR_DEAD: Color = Color::rgb(0.0, 0.0, 0.0);
const CELL_COLOR_ALIVE: Color = Color::rgb(0.0, 1.0, 0.0);

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_grid(mut commands: Commands, mut grid: ResMut<grid::Grid>) {

    let g: Vec<Vec<grid::Cell>> = grid.get().to_vec();


    //(WIDTH * -1.0) / 2.0) as f32, (HEIGHT / 2.0) as f32,

    for (idx1, i) in g.iter().enumerate(){
       for (idx2, j) in i.iter().enumerate() {


        commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: if j.is_alive() {
                    CELL_COLOR_ALIVE
                } else {
                    CELL_COLOR_DEAD
                },
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(CELL_SIZE, CELL_SIZE, 0.0),
                translation: Vec3::new(
                    idx2 as f32 * CELL_SIZE,
                    idx1 as f32 * CELL_SIZE * -1.0,
                    0.0,
                ),
                ..default()
            },
            ..default()
        })
        .insert(grid::Cell{
            alive: j.is_alive(),
            neighbors: grid.get_cell_neighbours(idx2, idx1),
        });


        let neighbors = grid.get_cell_neighbours(idx1, idx2);

        if neighbors < 2 {
            grid.kill_cell(idx2, idx1);
        }
        else if neighbors == 2 || neighbors == 3  {
            grid.revive_cell(idx2, idx1);
        }
        else if neighbors >= 4 {
            grid.kill_cell(idx2, idx1);
        }
        
       }
    }
}

fn spawn_cells(mut grid: ResMut<grid::Grid>){

    // let max_x: f32 = G_WIDTH / CELL_SIZE - 1.0;
    // let max_y: f32 = G_HEIGHT / CELL_SIZE - 1.0;

    // for _ in 0..500 {
    //     let x = rand::thread_rng().gen_range(0.0, max_x);
    //     let y = rand::thread_rng().gen_range(0.0, max_y);
    //     grid.revive_cell(x as usize, y as usize);
    // }
    // // grid.revive_cell(10 as usize, 10 as usize);
    // // grid.revive_cell(11 as usize, 10 as usize);
    // // grid.revive_cell(12 as usize, 10 as usize);
    

    
    //GLIDER//
    // [[0, 0, 1]
    // [1, 0, 1]
    // [0, 1, 1]]

    grid.revive_cell(12 as usize, 9 as usize);
    grid.revive_cell(10 as usize, 10 as usize);
    grid.revive_cell(12 as usize, 10 as usize);
    grid.revive_cell(11 as usize, 11 as usize);
    grid.revive_cell(12 as usize, 11 as usize);


    /*
    SPINNER
    [[0,0,0]
    [[1,1,1]
    [0,0,0]]

    grid.revive_cell(10 as usize, 10 as usize);
    grid.revive_cell(11 as usize, 10 as usize);
    grid.revive_cell(12 as usize, 10 as usize);
    */
}

fn main() {
    let r: f32 = G_WIDTH / CELL_SIZE;
    let c: f32 = G_HEIGHT / CELL_SIZE;

    let grid = grid::Grid::new(r as usize, c as usize);

    App::new()
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .insert_resource(WindowDescriptor {
            title: "Game of Life".to_string(),
            width: WIDTH,
            height: HEIGHT,
            ..default()
        })
        .add_startup_system(setup_camera)
        .insert_resource(grid)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.02))
                .with_system(spawn_grid),
        )
        .add_startup_system(spawn_cells)
        .add_plugins(DefaultPlugins)
        .run();
}
