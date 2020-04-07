/*
Consider this hypothetical situation: we work at a startup that’s making an app to generate
custom exercise workout plans. The backend is written in Rust, and the algorithm that generates
the workout plan takes into account many factors, such as the app user’s age, body mass index,
exercise preferences, recent workouts, and an intensity number they specify. The actual algorithm used
isn’t important in this example; what’s important is that this calculation takes a few seconds.
We want to call this algorithm only when we need to and only call it once so we don’t
make the user wait more than necessary.

We’ll simulate calling this hypothetical algorithm with the function `simulated_expensive_calculation`
*/

use std::thread;
use std::time::Duration;

pub fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}