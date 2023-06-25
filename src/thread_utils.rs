
use crate::RenderContext;
use crate::Vec3;
use crate::render_line;

use core::cmp::Ordering;
use std::thread;
use std::sync::mpsc;
use rand::rngs::SmallRng;

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
    pub fn new(srng: &SmallRng) -> (Dispatcher, mpsc::Receiver<RenderResult> ) {
        let mut handles = Vec::new();
        let mut command_transmitters = Vec::<mpsc::SyncSender<RenderCommand>>::new();

        let (render_tx, render_rx) = mpsc::sync_channel::<RenderResult>(1);

        for _ in 0..4 {
            // create new command tx/rx pairs. Store tx in the list, give rx to the thread.
            let (command_tx, command_rx) = mpsc::sync_channel::<RenderCommand>(1);
            // TODO: Pick appropriate command queue depth (or make it controllable, even)


            let mut srng = srng.clone();
            let threads_result_tx = render_tx.clone();

            let thread_handle = thread::spawn(move || {
                while let Ok(job) = command_rx.recv() {
                    match job {
                        RenderCommand::Stop => {
                            break;
                        }
                        RenderCommand::Line { line_num, context } => {
                            let line = render_line(line_num, &mut srng, context);
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
    pub fn submit_job(&mut self, command: RenderCommand) {
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
    }
}

