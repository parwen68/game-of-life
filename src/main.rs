extern crate rugra;
extern crate rand;
extern crate rayon;

use rugra::prelude::*;
use rand::{thread_rng, Rng};
use std::time::SystemTime;
use rayon::prelude::*;

// Initialize all cells
fn init(dim: isize) -> Vec<bool> {
    let mut rng = thread_rng();
    rng.gen_iter::<bool>().take((dim*dim) as usize).collect::<Vec<bool>>()
}

// Create a Rect for each cell
fn dot<'a>(x: f32, y: f32) -> Rect<'a> {
    let mut rect = Rect::new();
    rect.width(1f32);
    rect.height(1f32);
    rect.x(x);
    rect.y(y);
    rect
}

// Create all Rects
fn rects<'a>(dim: isize) -> Vec<Rect<'a>> {
    (0..dim*dim).map(|i| dot((i % dim) as f32, (i / dim) as f32)).collect::<Vec<Rect>>()
}

// From vector position to coordinate
fn to_coords(i: isize, dim: isize) -> (isize, isize){
    (i % dim, i / dim)
}

// From coordinate to vector position
fn from_coords(coords: (isize, isize), dim: isize) -> isize {
    coords.0 + coords.1 * dim
}

// Create a vector withh vectors for each cells neighbors
fn neigh(dim: isize) -> Vec<Vec<(isize)>> {
    let v: Vec<isize> = vec![-1,0,1];

    let r = v.iter().flat_map(|x|
        v.iter().map(move |y| (x,y))
    ).filter(|x| !(*x.0 == 0 && *x.1 == 0) )
        .collect::<Vec<_>>();

    (0..dim*dim)
        .map(|x| to_coords(x,dim))
        .map(|x|
            r.iter().map(move |y| ((x.0 as isize) + y.0, (x.1 as isize) + y.1))
                .filter(|x| x.0 >= 0 && x.1 >= 0 && x.0 < dim && x.1 < dim)
                .map(|x| from_coords(x, dim))
                .collect::<Vec<_>>()
        ).collect::<Vec<_>>()
}

// Calculate a new generation
fn next(v: Vec<(bool)>, neigh: &Vec<Vec<(isize)>>) -> Vec<(bool)> {

    v.par_iter().zip(neigh.par_iter()).map(|(x, y)| {
        let n = y.iter().filter(|z| v[**z as usize]).count();

        if *x && (n < 2 || n > 3) {
            false
        } else if !*x && n == 3 {
            true
        } else {
            *x
        }

    }).collect::<Vec<_>>()
}

// Update window
fn update(w: &mut Window) -> bool {
    w.to_sfml_window().display();
    w.is_open()
}


fn main() {
    let dim: isize = 1200;
    let mut window = Window::new("Game of Life");
    window.width(dim as u32).height(dim as u32);

    let mut cells = init(dim);
    let neigh = neigh(dim);
    let mut rects = rects(dim);

    let mut gen = 0;
    while update(&mut window) {
        let start = SystemTime::now();
        window.clear(0,0,0);

        let mut num = 0;
        for i in 0..dim*dim {
            if cells[i as usize] {
                rects[i as usize].draw(&mut window);
                num +=1
            }
        };
        cells = next(cells, &neigh);
        println!("{} ms, {} alive, {} generations", start.elapsed().unwrap().subsec_nanos() / (1000*1000), num, gen);
        gen +=1;
    }
}
