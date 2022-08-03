use bevy::core::FixedTimestep;
use bevy::prelude::*;
use rand::Rng;
use rand::thread_rng;
use rand::seq::SliceRandom;

pub mod grid;

const WIDTH: f32 = 850.0;
const HEIGHT: f32 = 850.0;

const CELL_SIZE: f32 = 4.0;


// ~ TODOOO: Add a new gradient ~ 
const CELL_COLOR_DEAD: Color = Color::rgb(0.0, 0.0, 0.0);

const CELL_COLOR_ALIVE_1: Color = Color::rgb(0.337, 0.0, 1.0); //rgb(86, 177, 255)
const CELL_COLOR_ALIVE_2: Color = Color::rgb(0.352, 0.0, 1.0); //rgb(90, 169, 255)
const CELL_COLOR_ALIVE_3: Color = Color::rgb(0.427, 0.0, 1.0); //rgb(109, 155, 255)
const CELL_COLOR_ALIVE_4: Color = Color::rgb(0.490, 0.0, 1.0); //rgb(125, 144, 255)
const CELL_COLOR_ALIVE_5: Color = Color::rgb(0.603, 0.0, 1.0); //rgb(154, 125, 255)
const CELL_COLOR_ALIVE_6: Color = Color::rgb(0.678, 0.0, 1.0); //rgb(173, 109, 255)
const CELL_COLOR_ALIVE_7: Color = Color::rgb(0.788, 0.0, 0.964); //rgb(201, 79, 246)
const CELL_COLOR_ALIVE_8: Color = Color::rgb(0.858, 0.0, 0.909); //rgb(219, 47, 232)


// const CELL_COLOR_ALIVE_1: Color = Color::rgb((254/255) as f32, (240/255) as f32, (1/255) as f32);
// const CELL_COLOR_ALIVE_2: Color = Color::rgb((255/255) as f32, (206/255) as f32, (3/255) as f32);
// const CELL_COLOR_ALIVE_3: Color = Color::rgb((253/255) as f32, (154/255) as f32, (1/255) as f32);
// const CELL_COLOR_ALIVE_4: Color = Color::rgb((253/255) as f32, (97/255) as f32, (4/255) as f32);
// const CELL_COLOR_ALIVE_5: Color = Color::rgb((255/255) as f32, (44/255) as f32, (5/255) as f32);
// const CELL_COLOR_ALIVE_6: Color = Color::rgb((240/255) as f32, (5/255) as f32, (5/255) as f32);

// const CELL_COLORS: [Color; 6] = [CELL_COLOR_ALIVE_1, CELL_COLOR_ALIVE_2, CELL_COLOR_ALIVE_3, CELL_COLOR_ALIVE_4, CELL_COLOR_ALIVE_5, CELL_COLOR_ALIVE_6];



// ~ TODO: Change setup_camera() -> setup() ~ 
fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn rotate(cell_colors: &mut Vec<Color>){
    &cell_colors.shuffle(&mut thread_rng());
}

fn spawn_grid(mut commands: Commands, mut grid: ResMut<grid::Grid>) {
    //println!("gen: {}", grid.get_gen());
    let mut cell_colors: Vec<Color> = vec![CELL_COLOR_ALIVE_1, CELL_COLOR_ALIVE_2, CELL_COLOR_ALIVE_3, CELL_COLOR_ALIVE_4, CELL_COLOR_ALIVE_5, CELL_COLOR_ALIVE_6, CELL_COLOR_ALIVE_7, CELL_COLOR_ALIVE_8];

    let g: Vec<Vec<grid::Cell>> = grid.get().to_vec();
    grid.increment_gen();
    rotate(&mut cell_colors);
    println!("{:?}", cell_colors);


    //random number from range 1 to 7

    //(WIDTH * -1.0) / 2.0) as f32, (HEIGHT / 2.0) as f32,

    for (idx1, i) in g.iter().enumerate(){
       for (idx2, j) in i.iter().enumerate() {

        let neighbors: i32 = grid.get_cell_neighbours(idx1, idx2);

        if neighbors < 1 {
            grid.kill_cell(idx2, idx1);
        }
        else if neighbors == 1 || neighbors == 2 {
            grid.revive_cell(idx2, idx1);
        }
        else if neighbors >= 3 {
            grid.kill_cell(idx2, idx1);
        }

        let color_index: usize = if neighbors <= 0 {1} else if neighbors >= 5 {5} else {neighbors as usize};

        commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: if j.is_alive() {
                    cell_colors[color_index]
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
        
       }
    }
}

fn despawn_system<M: Component>(
    mut commands: Commands, 
    query: Query<Entity, With<M>>
) {
    query.for_each(|entity| {
        commands.entity(entity).despawn();
    });
}

pub fn spawn_cells(mut grid: ResMut<grid::Grid>){

    let max_x: f32 = WIDTH / CELL_SIZE - 1.0;
    let max_y: f32 = HEIGHT / CELL_SIZE - 1.0;

    for _ in 0..500 {
        let x = rand::thread_rng().gen_range(0.0, max_x);
        let y = rand::thread_rng().gen_range(0.0, max_y);
        grid.revive_cell(x as usize, y as usize);
    }
    // grid.revive_cell(10 as usize, 10 as usize);
    // grid.revive_cell(11 as usize, 10 as usize);
    // grid.revive_cell(12 as usize, 10 as usize);
    // grid.revive_cell(13 as usize, 10 as usize);
    // grid.revive_cell(14 as usize, 10 as usize);
    

    
    //GLIDER//
    // [[0, 0, 1]
    // [1, 0, 1]
    // [0, 1, 1]]

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

fn main() {
    let r: f32 = WIDTH / CELL_SIZE;
    let c: f32 = HEIGHT / CELL_SIZE;

    let grid = grid::Grid::new(r as usize, c as usize);

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
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.025))
                .with_system(spawn_grid)
                .with_system(despawn_system::<grid::Cell>)
        )
        .add_startup_system(spawn_cells)
        .add_plugins(DefaultPlugins)
        .run();
}
