use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, BufReader};

type Graph = HashMap<u8, HashSet<u8>>;

fn read_line(s: &str) -> (u8, u8) {
    let s = s.as_bytes();
    (s[5], s[36])
}

fn read_graph<R: BufRead>(input: R) -> io::Result<Graph> {
    let mut graph = HashMap::new();
    for line in input.lines() {
        let line = read_line(&line?);
        graph.entry(line.0).or_insert_with(HashSet::new);
        graph
            .entry(line.1)
            .or_insert_with(HashSet::new)
            .insert(line.0);
    }

    Ok(graph)
}

fn next_step(graph: &Graph, done: &[u8]) -> u8 {
    let done = done.iter().cloned().collect::<HashSet<_>>();
    let mut ready = graph
        .iter()
        .filter_map(|(step, deps)| {
            if done.contains(step) {
                None
            } else if done.is_superset(deps) {
                Some(step)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    ready.sort();
    ready.reverse();
    ready.pop().unwrap().clone()
}

fn next_step_parallel(graph: &Graph, done: &[u8], in_progress: &[u8]) -> Option<u8> {
    let done = done.iter().cloned().collect::<HashSet<_>>();
    let in_progress = in_progress.iter().cloned().collect::<HashSet<_>>();
    let mut ready = graph
        .iter()
        .filter_map(|(step, deps)| {
            if done.contains(step) {
                None
            } else if done.is_superset(deps) {
                Some(step)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    ready.sort();
    ready.reverse();
    while let Some(step) = ready.pop() {
        if !in_progress.contains(step) {
            return Some(*step);
        }
    }
    None
}

fn order(graph: &Graph) -> String {
    let mut done = Vec::new();
    while done.len() < graph.len() {
        done.push(next_step(graph, &done));
    }
    done.into_iter().map(|b| b as char).collect()
}

fn step_time(step: u8) -> usize {
    61 + step as usize - ('A' as u8 as usize)
}

fn parallel_time_to_completion(graph: &Graph, workers: usize) -> usize {
    let mut time = 0;
    let mut done = Vec::new();
    let mut workers: Vec<Option<(u8, usize)>> = vec![None; workers];

    while done.len() < graph.len() {
        let mut in_progress = Vec::new();
        // Clear completed tasks
        workers = workers
            .into_iter()
            .map(|worker| match worker {
                Some((step, completion_time)) => {
                    if completion_time == time {
                        done.push(step);
                        None
                    } else {
                        in_progress.push(step);
                        Some((step, completion_time))
                    }
                }
                None => None,
            })
            .collect();
        // Assign new tasks
        workers = workers
            .into_iter()
            .map(|worker| match worker {
                Some((step, completion_time)) => Some((step, completion_time)),
                None => {
                    if let Some(step) = next_step_parallel(graph, &done, &in_progress) {
                        in_progress.push(step);
                        Some((step, time + step_time(step)))
                    } else {
                        None
                    }
                }
            })
            .collect();
        /*
        println!(
            "time {}, done {}, workers {:?}",
            time,
            done.iter().map(|b| *b as char).collect::<String>(),
            workers,
        );
        */
        time += 1;
    }

    time - 1
}

fn main() -> io::Result<()> {
    let graph = read_graph(BufReader::new(io::stdin()))?;

    // Part 1:
    println!("Part 1: {}", order(&graph));

    // Part 2:
    println!("Part 2: {}", parallel_time_to_completion(&graph, 5));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref GRAPH: Graph = read_graph(BufReader::new(
            &b"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."[..]
        ))
        .unwrap();
    }

    #[test]
    fn test_read_line() {
        assert_eq!(
            read_line("Step C must be finished before step A can begin."),
            ('C' as u8, 'A' as u8)
        );
        assert_eq!(
            read_line("Step B must be finished before step E can begin."),
            ('B' as u8, 'E' as u8)
        );
    }

    fn dep_string(c: char) -> String {
        let mut deps = GRAPH
            .get(&(c as u8))
            .unwrap()
            .iter()
            .cloned()
            .map(|b| b as char)
            .collect::<Vec<_>>();
        deps.sort();
        deps.into_iter().collect()
    }

    #[test]
    fn test_read_graph() {
        assert_eq!(GRAPH.len(), 6);
        assert_eq!(dep_string('C'), "");
        assert_eq!(dep_string('A'), "C");
        assert_eq!(dep_string('F'), "C");
        assert_eq!(dep_string('B'), "A");
        assert_eq!(dep_string('D'), "A");
        assert_eq!(dep_string('E'), "BDF");
    }

    #[test]
    fn test_next_step() {
        assert_eq!(next_step(&GRAPH, &[]), 'C' as u8);
        assert_eq!(next_step(&GRAPH, &['C' as u8]), 'A' as u8);
        assert_eq!(next_step(&GRAPH, &['C' as u8, 'A' as u8]), 'B' as u8);
        //assert_eq!(next_step(&GRAPH, vec![].into_iter().collect()), 'C');
    }

    #[test]
    fn test_order() {
        assert_eq!(order(&GRAPH), "CABDFE");
    }

    #[test]
    fn test_step_time() {
        assert_eq!(step_time('A' as u8), 61);
        assert_eq!(step_time('Z' as u8), 86);
    }
}
