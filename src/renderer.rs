
use crate::primitives::{Vec3, Ray, Rect};
use crate::scene::{
    Camera,
    Hittable,
};

use core::cmp::Ordering;
use std::thread;
use std::sync::mpsc;
use std::ops;
use rand::Rng;
use rand::rngs::SmallRng;
use rand::distributions::Uniform;
use itertools::Itertools;

// =================
// Description parts
// =================

#[derive (Clone)]
pub struct RenderContext{
    pub image: (i32, i32),
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub world: Hittable,
    pub camera: Camera,
}

pub struct DistributionContianer {
    pub distrib_zero_one: Uniform<f32>,
    pub distrib_plusminus_one: Uniform<f32>,
}

impl DistributionContianer {
    fn new() -> Self {
        DistributionContianer {
            distrib_zero_one: Uniform::new(0.0, 1.0),
            distrib_plusminus_one: Uniform::new(-1.0, 1.0),
        }
    }
}

// =============
// Drawing Parts
// =============

fn render_line(y: i32, small_rng: &mut SmallRng, context: RenderContext, distr: &DistributionContianer) -> Vec<Vec3> {
    //TODO: Ensure that the compiler hoists the distribution's out as constants
    // else, do so manually
   (0..context.image.0).map(|x| {
       sample_pixel(x, y, small_rng, &context, distr)
    }).collect()
}

fn ray_color(r: Ray, world: &Hittable, depth: u32, srng: &mut SmallRng, distrib: Uniform<f32> ) -> Vec3 {
    // recursion depth guard
    if depth == 0 {
        return Vec3::zero();
    }

    if let Some(rec) = world.hit(r, 0.001, f32::INFINITY){
        let mut scattered = Ray {
            orig: Vec3::zero(),
            dir: Vec3::zero()
        };
        let mut attenuation = Vec3::zero();
        if rec.material.scatter(r, &rec, &mut attenuation, &mut scattered, srng) {
            return attenuation * ray_color(scattered, world, depth-1, srng, distrib);
        };
    }

    let unitdir = Vec3::as_unit(r.dir);
    let t = 0.5 * (unitdir.y + 1.0);
    return Vec3::ones() * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn sample_pixel(x: i32, y: i32, small_rng: &mut SmallRng, context: &RenderContext, distr: &DistributionContianer) -> Vec3{
    (0..context.samples_per_pixel).into_iter().fold(
        Vec3::zero(),
        |color, _sample| {
            let u = ((x as f32) + small_rng.sample(distr.distrib_zero_one)) / ((context.image.0 - 1) as f32);
            let v = ((y as f32) + small_rng.sample(distr.distrib_zero_one)) / ((context.image.1 - 1) as f32);
            let ray = context.camera.get_ray(u, v, small_rng);
            color + ray_color(ray, &context.world, context.max_depth, small_rng, distr.distrib_plusminus_one)
        }
    )
}

// ===============
// Execution parts
// ===============

/* Iterable that produces pixels left-to-right, top-to-bottom.
 * `Tile`s represent the render space, not the finished image.
 * There is no internal pixel buffer
 */

type TileCursorIter = itertools::Product<ops::Range<i32>, ops::Range<i32>>;

struct Tile {
    bounds: Rect,
    context: RenderContext,
    small_rng: SmallRng,
    rand_distr: DistributionContianer,
    cursor: TileCursorIter,
}

impl Tile{
    fn new(
        bounds: Rect,
        context: RenderContext,
        small_rng: SmallRng,
        rand_distr: DistributionContianer
    ) -> Self
    {
        Tile { bounds, context, small_rng, rand_distr,
            cursor: (bounds.x..(bounds.x + bounds.w))
                .cartesian_product(bounds.y..(bounds.y + bounds.h)
            )
        }

    }
}

impl Iterator for Tile {
    type Item = Vec3;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((x, y)) = self.cursor.next(){
            Some(sample_pixel(
                x, y,
                &mut self.small_rng,
                &self.context,
                &self.rand_distr,
            ))
        } else {
            None
        }
    }
}



#[derive (Clone)]
pub enum RenderCommand{
    Stop,
    Line { line_num: i32, context: RenderContext },
}

pub struct RenderResult {
    pub line_num: i32,
    pub line: Vec<Vec3>,
}

impl Ord for RenderResult {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.line_num > other.line_num {
            Ordering::Less
        } else if self.line_num < other.line_num {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for RenderResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for RenderResult {
    fn eq(&self, other: &Self) -> bool {
        self.line_num == other.line_num
    }
}

impl Eq for RenderResult {}

/*
 *  The dispatcher will hold a list of threads, and a list of command input channels to match.
 *  Helper functions exist to input jobs serially, and then dispatch them to an open thread.
 *
 *  Since receivers can be matched to several senders, the input end of the result channel will
 *  be cloned and given to each of the threads.
 *  TODO: Consider holding a copy of the render_tx end in case threads exit early and need to
 *  be restored.
 */
pub struct Dispatcher{
    handles: Vec<thread::JoinHandle<()>>,
    command_transmitters: Vec<mpsc::SyncSender<RenderCommand>>, 
    next_to_feed: usize, // gonna do a round-robin style dispatch, ig.
}

impl Dispatcher {
    pub fn new(srng: &SmallRng, num_threads: usize) -> (Dispatcher, mpsc::Receiver<RenderResult> ) {
        let mut handles = Vec::new();
        let mut command_transmitters = Vec::<mpsc::SyncSender<RenderCommand>>::new();

        let (render_tx, render_rx) = mpsc::sync_channel::<RenderResult>(1);

        for _ in 0..num_threads {
            // create new command tx/rx pairs. Store tx in the list, give rx to the thread.
            let (command_tx, command_rx) = mpsc::sync_channel::<RenderCommand>(1);
            // TODO: Pick appropriate command queue depth (or make it controllable, even)


            let mut srng = srng.clone();
            let threads_result_tx = render_tx.clone();
            let distribs = DistributionContianer::new();
            let thread_handle = thread::spawn(move || {
                while let Ok(job) = command_rx.recv() {
                    match job {
                        RenderCommand::Stop => {
                            break;
                        }
                        RenderCommand::Line { line_num, context } => {
                            let line = render_line(line_num, &mut srng, context, &distribs);
                            let result = RenderResult { line_num, line };
                            threads_result_tx.send(result).unwrap();
                        }
                    }
                }
            });
            handles.push(thread_handle);
            command_transmitters.push(command_tx);
        }
        // finally, stash everything in the Dispatcher struct and return.
        (
            Dispatcher{
                handles,
                command_transmitters,
                next_to_feed: 0,
            },
            render_rx
        )
    }

    //TODO: Reconsider round-robin dispatch
    // When passing the message to threads which are still busy, this function
    // will block (it's a sync_channel). While blocked, other threads could
    // become available and left idle.
    pub fn submit_job(&mut self, command: RenderCommand) -> Result<(), mpsc::SendError<RenderCommand>> {
        // Stop command is special. We'll broadcast it to all threads.
        if let RenderCommand::Stop = command {
            for channel in &self.command_transmitters {
                return channel.send(command.clone());
            }
        }

        // Check that `next_to_feed` is in-bounds, and then insert.
        // index is post-incremented with this function call.

        // wrap when at length (0-indexed so last valid index is len-1)
        if self.next_to_feed == self.handles.len() {
            self.next_to_feed = 0;
        } else if self.next_to_feed > self.handles.len() {
            panic!("How the hell did a +=1 skip past the maximum allowed size?");
        }

        match self.command_transmitters.get(self.next_to_feed){
            Some(target) => target.send(command).unwrap(),
            None => panic!("oh god oh fuck"),
        }
        self.next_to_feed += 1;
        Ok(())
    }
}

