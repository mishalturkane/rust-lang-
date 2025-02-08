fn main() {
    let mut count = 0;

    // Closure that gets called each time the timer ticks
    let timer_tick = || {
        count += 1;
        println!("Timer ticked! Count: {}", count);
    };

    // Simulating a timer that ticks 5 times
    for _ in 0..5 {
        timer_tick();  // Calling the closure each time the timer ticks
    }
}
