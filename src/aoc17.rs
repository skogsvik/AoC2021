use crate::loaders::file_to_string;
use itertools::Itertools;
use std::collections::HashMap;

pub const DATA: &str = "input/aoc17";

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Target {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

pub fn load(filename: impl AsRef<std::path::Path>) -> Target {
    parse_input(&file_to_string(filename))
}

fn parse_input(input: &str) -> Target {
    let mut coords = input[13..] // Skip prefix junk
        .trim_end() // Skip any whitespace at the end
        .split(", ") // Split into each axis
        .flat_map(|axis| axis[2..].split("..")) // Split into range parts
        .map(|coord| coord.parse().unwrap());
    Target {
        x_min: coords.next().unwrap(),
        x_max: coords.next().unwrap(),
        y_min: coords.next().unwrap(),
        y_max: coords.next().unwrap(),
    }
}

pub fn answer1(input: &Target) -> i32 {
    /*
    It always takes `2*Vy(t=0)+1` time to return to the start height when firing upward.
    The maximum start velocity is thus decided such that one time step later the probe is at the
    very bottom of the target, i.e.
    `y_target_min - y(t=0) = Vy(t=2*Vy(t=0)+1)`.
    Since
    `Vy(t) = Vy(t=0) - t` we have
    `max Vy(t=0) = y(t=0) - y_target_min - 1`.
    */
    let max_vel = -input.y_min - 1;
    /*
    The maximum height is reached halfway before returning to start, i.e. at time `Vy(t=0)`.
    Since
    `y(t) = y(t=0) + t*Vy(t=0) + (t-t^2)/2` the maximum height is
    `y(t=Vy(t=0)) = y(t=0) + (Vy(t=0)^2 + Vy(t=0))/2`
    */
    (max_vel * max_vel + max_vel) / 2
}

/// Find all the time slots when probe is within target y-coordinates for each valid y-velocity
/// The time to a given position is given by
/// `y(t) = Vx(t=0) * t + (t^2-t)/2` <=>
/// t = Vx(t=0) + 0.5 ± sqrt((Vx(t=0) + 0.5)^2 - 2*y(t))
/// Where we discard the negative solution as it always gives negative time when y(t) < 0
fn calc_y_times_per_vel(target: &Target) -> Vec<Vec<i32>> {
    // The lowest valid starting velocity is y_min since it reaches the bottom of target in a single
    // step to the.
    // The highest valid starting velocity is given in answer 1
    (target.y_min..=-target.y_min - 1) // For each potentially valid velocity
        .map(|vel| {
            let prefix = vel as f32 + 0.5;
            let [time_to_min, time_to_max] = [target.y_min, target.y_max]
                .map(|target| prefix + (prefix * prefix - (2 * target) as f32).sqrt());

            // The times inside target is the time between entering the top and exiting the bottom
            (time_to_max.ceil() as i32..=time_to_min.floor() as i32).collect()
        })
        .collect()
}

/// The required starting velocity to come to a full stop at a certain coordinate.
/// Given by
/// `y(t) = Vx(t=0) * t + (t^2-t)/2`
/// but swapping t for Vx(t=0) as it is  the time it takes to come to a stop, i.e.
/// `y(t>=Vx(t=0)) = Vx(t=0)^2 + (Vx(t=0)^2-Vx(t=0))/2`
/// =>
/// `Vx(t=0) = (±sqrt(8 * y(t>=Vx(t=0)) + 1) - 1)/2`
/// Discarding the negative solution as it has negative velocity
fn velocity_to_stop_at(x: i32) -> f32 {
    (((8 * x + 1) as f32).sqrt() - 1.) / 2.
}

/// The required starting velocity to be at the certain coordinate at a certain time.
/// Given by solving
/// `y(t) = Vx(t=0) * t + (t^2-t)/2`
/// <=>
/// `Vx(t=0) = (2*y(t) + t^2-t)/(2*t)`
fn velocity_to_reach_on_time(x: i32, time: f32) -> f32 {
    ((2 * x) as f32 + time * time - time) / (2. * time)
}

/// Find all the x-velocities which place probe within target x-coordinates for each time slot
/// Target can be reached in the targeted time in 2 different ways:
/// * Right on time without slowing to a stop
/// * Stopping at the position at any time before (or including) the given time slot
/// The applicable option is decided by the target time
fn calc_x_times_and_velocities<'a>(
    target: &Target,
    times: impl Iterator<Item = &'a i32>,
) -> HashMap<i32, Vec<i32>> {
    // Velocity needed to stop just within the left target border
    let min_stop_velocity = velocity_to_stop_at(target.x_min).ceil() as i32;
    // Velocity needed to stop just within the right target border
    let max_stop_velocity = velocity_to_stop_at(target.x_max).floor() as i32;
    times
        .map(|time| {
            // Velocity needed to reach within the left target border right on time
            let velocity_to_hit_x_min =
                velocity_to_reach_on_time(target.x_min, *time as f32).ceil() as i32;
            // Velocity needed to reach within the right target border right on time
            let velocity_to_hit_x_max =
                velocity_to_reach_on_time(target.x_max, *time as f32).floor() as i32;

            // Combine all possible velocities
            // Note that stopping requires at least Vx(t=0) time and reaching just-in-time requires
            // at most Vx(t=0) time, therefore anything outside this needs to be removed.
            let velocities = (min_stop_velocity..=max_stop_velocity.min(*time))
                .chain(velocity_to_hit_x_min.max(*time)..=velocity_to_hit_x_max)
                .collect();

            (*time, velocities)
        })
        .collect()
}

pub fn answer2(input: &Target) -> usize {
    // Find all the time slots when probe is within target y-coordinates for each valid y-velocity
    let y_times_per_vel = calc_y_times_per_vel(input);
    // All the time slots when the probe may be in target for at leas one y-velocity
    let all_times = y_times_per_vel.iter().flatten().unique();
    // Find all the x-velocities which place probe within target x-coordinates for each time slot
    let x_times_and_velocities = calc_x_times_and_velocities(input, all_times);

    // Match up valid y-velocities and x-velocities by time slot to see if they hit target
    y_times_per_vel
        .iter()
        .map(|times| {
            times
                .iter()
                // All x-velocities which places the probe within target x-coordinates while it is
                // also within y-coordinates
                .flat_map(|time| &x_times_and_velocities[time])
                // Filter out repeated velocities due to probe remaining within target for multiple
                // time slots
                .unique()
                .count() // Total number of unique x velocities for this y velocity
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_DATA: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_answer1_mock_data() {
        assert_eq!(answer1(&parse_input(MOCK_DATA)), 45)
    }

    #[test]
    fn test_answer2_mock_data() {
        assert_eq!(answer2(&parse_input(MOCK_DATA)), 112)
    }

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(&load(DATA)), 5995)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(&load(DATA)), 3202)
    }
}
