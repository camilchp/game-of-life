extern crate sdl2;

use rand::Rng;

use std::{thread, time};
use sdl2::pixels::Color;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect};
use sdl2::render::{WindowCanvas};
use sdl2::EventPump;

const MAX_X : usize = 150;
const MAX_Y : usize = 150;
const CELL_SIZE : usize = 4;

fn init() -> (WindowCanvas, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Conway's Game of Life", (MAX_X*CELL_SIZE) as u32, (MAX_Y*CELL_SIZE) as u32)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let event_pump = sdl_context.event_pump().unwrap();

    //let mut renderer = window.renderer().build().unwrap();
    //renderer.set_draw_color(Color::RGB(255, 255, 255));
    //renderer.clear();
    //renderer.present();

    let canvas = window.into_canvas().build().expect("could not make canvas");

    (canvas, event_pump)
}

fn neighbour_count(board : &[[bool;MAX_Y];MAX_X], i : usize, j : usize) -> u8 {
    let mut count = 0;
    let mut neighbours = vec![(i+1, j), (i+1, j+1), (i, j+1)];
    if i > 0 {neighbours.push((i-1, j+1)); neighbours.push((i-1, j));} else {}
    if j > 0 {neighbours.push((i, j-1)); neighbours.push((i+1, j-1));} else {}
    if i > 0 && j > 0 {neighbours.push((i-1, j-1))} else {}
    //println!("{:?}", neighbours);
    for (p, q) in neighbours.iter() {
        if *p >= MAX_X || *q >= MAX_Y {} else {if board[*p][*q] {count += 1} else {}}
    }

    count
}

fn step(board : [[bool;MAX_Y];MAX_X]) -> [[bool;MAX_Y];MAX_X]{
    let mut newboard = board.clone();
    for i in 0..MAX_X  {
        for j in 0..MAX_Y {
            //println!("({},{}) : {}", i, j, neighbour_count(&board, i, j));
            if board[i][j] {
                match neighbour_count(&board, i, j) {
                    2 | 3 => (),
                    _ => newboard[i][j] = false,
                }
            } else {
                if neighbour_count(&board, i, j) == 3 {
                    newboard[i][j] = true
                }
            }
        }
    }
    newboard
}

fn display_cell(canvas : &mut WindowCanvas, i : usize, j : usize) {
    canvas.fill_rect(Rect::new((i*CELL_SIZE) as i32, (j*CELL_SIZE) as i32, CELL_SIZE as u32, CELL_SIZE as u32)).unwrap();
}

fn display_board(canvas : &mut WindowCanvas, board : &[[bool;MAX_Y];MAX_X]) {
    for i in 0..MAX_X {
        for j in 0..MAX_Y {
            if board[i][j] {display_cell(canvas, i, j)} else {}
        }
    }
}

fn random_board() -> [[bool;MAX_Y];MAX_X] {
    let mut board = [[false;MAX_Y];MAX_X];
    let mut rng = rand::thread_rng();
    for i in 0..MAX_X {
        for j in 0..MAX_Y {
            let x : f32 = rng.gen();
            if x > 0.8 {board[i][j] = true;} else {}
        }
    }
    board
}
fn main() {

    let (mut canvas, mut event_pump) = init();

    let mut board = [[false;MAX_Y];MAX_X];
    board[0+5][1+5] = true;
    board[1+5][2+5] = true;
    board[2+5][0+5] = true;
    board[2+5][1+5] = true;
    board[2+5][2+5] = true;

    let mut play = true;

    'running:loop {
        let event = event_pump.poll_event();
            match event {
                Some(Event::Quit {..}) | Some(Event::KeyDown {
                    keycode : Some(Keycode::Escape), ..
                }) => {break 'running},

                Some(Event::KeyDown {
                    keycode : Some(Keycode::Space), ..
                }) => {play = !play},

                Some(Event::KeyDown {
                    keycode : Some(Keycode::Backspace), ..
                }) => {board = [[false;MAX_Y];MAX_X]},

                Some(Event::MouseButtonDown {
                    x : a, y : b, ..
                }) => {
                    let x = (a as usize)/CELL_SIZE;
                    let y = (b as usize)/CELL_SIZE;
                    board[x][y] = !board[x][y];
                }

                Some(Event::KeyDown {
                    keycode : Some(Keycode::P), ..
                }) => {println!("{:?}", board)},
                
                Some(Event::KeyDown {
                    keycode : Some(Keycode::C), ..
                }) => {board = [[false;MAX_Y];MAX_X];},

                Some(Event::KeyDown {
                    keycode : Some(Keycode::R), ..
                }) => {board = random_board();},

                _ => {},
            }
            canvas.clear();
            //println!("{:?}", board);
            //println!("");
            //println!("{:?}, {:?}", board[1][2], board[0][1]);
            if play {board = step(board);} else {}
            
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            display_board(&mut canvas, &board);
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.present();

        thread::sleep(time::Duration::from_millis(5));
    }
}