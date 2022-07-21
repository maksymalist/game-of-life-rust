use std::ops::Add;
use std::usize;

use bevy::core::FixedTimestep;
use bevy::prelude::*;
pub mod grid;


const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;
const CELL_SIZE: f32 = 10.0;


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

        let neighbors = grid.get_cell_neighbours(idx1, idx2);

        if neighbors < 2 {
            grid.kill_cell(idx2, idx1);
        }
        else if neighbors == 2 || neighbors == 3  {
            grid.revive_cell(idx2, idx1);
        }
        else if neighbors > 3 {
            grid.kill_cell(idx2, idx1);
        }

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
                    (WIDTH * -1.0)/ 2.0 + (idx2 as f32 * CELL_SIZE),
                    (HEIGHT / 2.0) - (idx1 as f32 * CELL_SIZE),
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
       }
    }
}

fn spawn_cells(mut grid: ResMut<grid::Grid>){

    println!("revived cells");
    grid.revive_cell(12 as usize, 9 as usize);
    grid.revive_cell(10 as usize, 10 as usize);
    grid.revive_cell(12 as usize, 10 as usize);
    grid.revive_cell(11 as usize, 11 as usize);
    grid.revive_cell(12 as usize, 11 as usize);
    

    /*
    //GLIDER//
    [[0, 0, 1]
    [1, 0, 1]
    [0, 1, 1]]
    */
}

fn main() {
    let r: f32 = WIDTH / CELL_SIZE;
    let c: f32 = HEIGHT / CELL_SIZE;

    let grid = grid::Grid::new(r as usize, c as usize);

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
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
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(spawn_grid),
        )
        .add_startup_system(spawn_cells)
        .add_plugins(DefaultPlugins)
        .run();
}
