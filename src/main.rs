use std::ops::Range;
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use rand::Rng;
use bevy_inspector_egui::{InspectorPlugin, Inspectable};

pub mod grid;

const WIDTH: f32 = 850.0;
const HEIGHT: f32 = 850.0;

const CELL_SIZE: f32 = 2.0;
const TICK_RATE : f64 = 0.1;
const CELL_COLOR_DEAD: Color = Color::rgb(0.0, 0.0, 0.0);

// ~ TODO: Change setup_camera() -> setup() ~ 
fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn randomize(data: &mut InspectableData){
    let random_range_1: i32 = rand::thread_rng().gen_range(2, 4);
    let random_range_2: i32 = rand::thread_rng().gen_range(random_range_1+1, 8);
    let random_range: Range<i32> = random_range_1..random_range_2;
    data.min_to_revive = random_range;
    data.min_to_die = rand::thread_rng().gen_range(2, 4);
    data.max_to_die = rand::thread_rng().gen_range(1, 6);
}

fn rainbow_colors(data: &mut InspectableData){
    data.color = Color::rgb(rand::thread_rng().gen_range(0.0, 1.0), rand::thread_rng().gen_range(0.0, 1.0), rand::thread_rng().gen_range(0.0, 1.0));
}

fn spawn_grid(mut commands: Commands, mut grid: ResMut<grid::Grid>, mut data: ResMut<InspectableData>) {
    if data.pause {
        return;
    }
    let g: Vec<Vec<grid::Cell>> = grid.get().to_vec();
    grid.increment_gen();
    //random number from range 1 to 7

    //(WIDTH * -1.0) / 2.0) as f32, (HEIGHT / 2.0) as f32,

    let mut cataclysm: bool = true;
    for (idx1, i) in g.iter().enumerate(){
       for (idx2, j) in i.iter().enumerate() {

        let neighbors: i32 = grid.get_cell_neighbours(idx1, idx2);

        // min neighbor = 1
        if neighbors < data.min_to_die {
            grid.kill_cell(idx2, idx1);
        }

        // min neibors to revive = 2 || 1
        else if data.min_to_revive.contains(&neighbors)  {
            grid.revive_cell(idx2, idx1);
        }
        // min neighbors to kill = 3 || 4 || 5 || 6 || 7 || 8
        else if neighbors >= data.max_to_die {
            grid.kill_cell(idx2, idx1);
        }

        if j.is_alive() {
            cataclysm = false;
        }

        commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: if j.is_alive() {
                    data.color
                } else {
                    CELL_COLOR_DEAD
                },
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(CELL_SIZE, CELL_SIZE, 0.0),
                translation: Vec3::new(
                    idx2 as f32 * CELL_SIZE - (WIDTH / 2.0),
                    idx1 as f32 * CELL_SIZE * -1.0 + HEIGHT / 2.0,
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

        let neighbors: i32 = grid.get_cell_neighbours(idx1, idx2);

        // min neighbor = 1
        if neighbors < data.min_to_die {
            grid.kill_cell(idx2, idx1);
        }

        // min neibors to revive = 2 || 1
        else if data.min_to_revive.contains(&neighbors)  {
            grid.revive_cell(idx2, idx1);
        }
        // min neighbors to kill = 3 || 4 || 5 || 6 || 7 || 8
        else if neighbors >= data.max_to_die {
            grid.kill_cell(idx2, idx1);
        }

        if j.is_alive() {
            cataclysm = false;
        }

        
       }
    }
    if cataclysm && data.infinit {
        let max_x: f32 = WIDTH / CELL_SIZE - 1.0;
        let max_y: f32 = HEIGHT / CELL_SIZE - 1.0;
    
        for _ in 0..100 {
            let x = rand::thread_rng().gen_range(0.0, max_x);
            let y = rand::thread_rng().gen_range(0.0, max_y);
            grid.revive_cell(x as usize, y as usize);
        }
    }
    if data.rainbow && grid.get_gen() % 16 == 0 {
        rainbow_colors(&mut data);
    }
    if data.random && grid.get_gen() % 16 == 0 {
        randomize(&mut data)
    }
}

fn despawn_system<M: Component>(
    mut commands: Commands, 
    query: Query<Entity, With<M>>,
    data: Res<InspectableData>
) {
    if data.pause {
        return;
    }
    query.for_each(|entity| {
        commands.entity(entity).despawn();
    });
}

pub fn draw(mut grid: ResMut<grid::Grid>, matrix: Vec<Vec<usize>>, start_x: usize, start_y: usize) {

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 1 {
                grid.revive_cell(j + start_x, i+start_y);
            }
        }
    }

}

pub fn spawn_cells(mut grid: ResMut<grid::Grid>){

    let max_x: f32 = WIDTH / CELL_SIZE - 1.0;
    let max_y: f32 = HEIGHT / CELL_SIZE - 1.0;

    // for _ in 0..1000 {
    //     let x = rand::thread_rng().gen_range(0.0, max_x);
    //     let y = rand::thread_rng().gen_range(0.0, max_y);
    //     grid.revive_cell(x as usize, y as usize);
    // }

    let glider = vec![
        vec![0, 1, 1, 0],
        vec![0, 0, 0, 0],
        vec![1, 0, 0, 1],
        vec![0, 0, 0, 0],
    ];

    draw(grid, glider, (max_x/2.0)as usize, (max_y/2.0)as usize);
    // grid.revive_cell(10 as usize, 10 as usize);
    // grid.revive_cell(11 as usize, 10 as usize);
    // grid.revive_cell(12 as usize, 10 as usize);
    // grid.revive_cell(13 as usize, 10 as usize);
    // grid.revive_cell(14 as usize, 10 as usize);
    

    
    //GLIDER//
    // [0, 0, 1, 0]
    // [1, 0, 0, 0]
    // [1, 0, 0, 0]
    // [0, 0, 1, 0]

    // grid.revive_cell(12 as usize, 9 as usize);
    // grid.revive_cell(10 as usize, 10 as usize);
    // grid.revive_cell(12 as usize, 10 as usize);
    // grid.revive_cell(11 as usize, 11 as usize);
    // grid.revive_cell(12 as usize, 11 as usize);


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

#[derive(Inspectable)]
struct InspectableData {
    color: Color,
    min_to_die: i32,
    min_to_revive: Range<i32>,
    max_to_die: i32,
    random: bool,
    infinit: bool,
    rainbow: bool,
    pause: bool,
}


impl Default for InspectableData {
    fn default() -> Self {
        Self {
            color: Color::rgb(0.337, 0.0, 1.0),
            min_to_die: 2,
            min_to_revive: 2..3,
            max_to_die: 1,
            random: false,
            infinit: false,
            rainbow: false,
            pause: false,
        }
    }
}


fn main() {
    let r: f32 = WIDTH / CELL_SIZE;
    let c: f32 = HEIGHT / CELL_SIZE;

    let grid = grid::Grid::new(r as usize, c as usize);
    let data = InspectableData::default();

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            title: "Game of Life".to_string(),
            width: WIDTH,
            height: HEIGHT,
            ..default()
        })
        .add_startup_system(setup_camera)
        .insert_resource(grid)
        .insert_resource(data)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TICK_RATE))
                .with_system(spawn_grid)
                .with_system(despawn_system::<grid::Cell>)
        )
        .add_startup_system(spawn_cells)
        .add_plugins(DefaultPlugins)
        .add_plugin(InspectorPlugin::<InspectableData>::new())
        .run();
}
