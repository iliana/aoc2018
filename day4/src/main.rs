use day4::What;
use failure::Fallible;
use std::collections::HashMap;
use std::mem;

fn main() -> Fallible<()> {
    // < guard ID, ( minutes asleep, [ times asleep each minute ] ) >
    let mut data: HashMap<u16, (usize, [usize; 60])> = HashMap::new();
    let mut events = day4::read_events().collect::<Result<Vec<_>, _>>()?;
    events.sort_unstable();
    events.reverse();

    let mut current_guard = if let What::Begins(id) = events.pop().unwrap().what {
        id
    } else {
        panic!();
    };
    let mut asleep_since = {
        let event = events.pop().unwrap();
        assert!(event.what == What::FallsAsleep);
        Some(event.time)
    };

    while let Some(event) = events.pop() {
        match event.what {
            What::Begins(id) => {
                current_guard = id;
            }
            What::FallsAsleep => {
                asleep_since = Some(event.time);
            }
            What::WakesUp => {
                if let Some(asleep_since) = mem::replace(&mut asleep_since, None) {
                    let entry = data.entry(current_guard).or_insert((0, [0; 60]));
                    entry.0 += (event.time - asleep_since) as usize;
                    for time in asleep_since.range(event.time) {
                        entry.1[time.minute as usize] += 1;
                    }
                }
            }
        }
    }

    // Who slept the most?
    let sleepiest_guard = data
        .iter()
        .map(|(guard, (minutes, _))| (minutes, guard))
        .max()
        .unwrap()
        .1;
    // What minute did they sleep the most?
    let sleepiest_minute = data
        .get(sleepiest_guard)
        .unwrap()
        .1
        .iter()
        .enumerate()
        .map(|(i, n)| (n, i))
        .max()
        .unwrap()
        .1;
    println!("{}", (*sleepiest_guard as usize) * sleepiest_minute);

    // Who slept most consistently?
    let (_, minute, guard) = data
        .iter()
        .map(|(guard, (_, minutes))| {
            let (n, minute) = minutes
                .iter()
                .enumerate()
                .map(|(minute, n)| (n, minute))
                .max()
                .unwrap();
            (n, minute, guard)
        })
        .max()
        .unwrap();
    println!("{}", minute * (*guard as usize));

    Ok(())
}
