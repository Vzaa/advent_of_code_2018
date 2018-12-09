use std::char::ParseCharError;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug)]
struct Edge {
    u: char,
    v: char,
}

#[derive(Debug)]
enum ParseEdgeError {
    Char(ParseCharError),
    Empty,
}

impl From<ParseCharError> for ParseEdgeError {
    fn from(e: ParseCharError) -> Self {
        ParseEdgeError::Char(e)
    }
}

impl FromStr for Edge {
    type Err = ParseEdgeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_whitespace();
        let u = it.nth(1).ok_or(ParseEdgeError::Empty)?.parse()?;
        let v = it.nth(5).ok_or(ParseEdgeError::Empty)?.parse()?;
        Ok(Edge { u, v })
    }
}

#[derive(Debug, Clone)]
struct Worker {
    busy_until: i32,
    task: Option<char>,
}

impl Worker {
    pub fn new() -> Worker {
        Worker {
            busy_until: -1,
            task: None,
        }
    }

    fn work(&mut self, time: i32, task: char) {
        assert!(self.avail(time));

        let amount: i32 = ((task as u8) - ('A' as u8) + 1) as i32;
        self.busy_until = time + 60 + amount;
        self.task = Some(task);
    }

    fn avail(&self, time: i32) -> bool {
        self.busy_until <= time
    }

    fn output(&mut self, time: i32) -> Option<char> {
        if !self.avail(time) {
            return None;
        }

        // Return if there is a task
        let tmp = self.task;
        self.task = None;
        tmp
    }
}

fn main() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let edges: Vec<Edge> = rdr.lines().map(|l| l.unwrap().parse().unwrap()).collect();

    let nodes: HashSet<char> = edges
        .iter()
        .map(|x| x.u)
        .chain(edges.iter().map(|x| x.v))
        .collect();

    // Going to determine starting nodes by removing all v in edges u -> v
    let mut starting = nodes.clone();

    let mut deps_org = HashMap::new(); // dependencies
    for e in &edges {
        let elist_rev = deps_org.entry(e.v).or_insert(HashSet::new());

        starting.remove(&e.v);
        elist_rev.insert(e.u);
    }

    // We could also handle multiple starting points but the problem has one so just assert
    assert_eq!(starting.len(), 1);
    let start = *starting.iter().next().unwrap();

    // Part 1
    {
        let mut deps = deps_org.clone();
        let mut visited = Vec::new();
        let mut vis = start;

        loop {
            visited.push(vis);
            deps.remove(&vis);

            // Remove visited node from dependencies of others
            for d in deps.values_mut() {
                d.remove(&vis);
            }

            let next = deps
                .iter()
                .filter(|kv| kv.1.is_empty()) // Get nodes with no dependencies left
                .min_by_key(|kv| kv.0); // Sort by alphabetical

            if let Some((id, _)) = next {
                vis = *id;
            } else {
                assert!(deps.is_empty());
                // We're done, no more left
                break;
            }
        }

        let path: String = visited.iter().collect();
        println!("Part 1, Path is: {}", path);
    }

    // Part 2
    {
        let mut deps = deps_org.clone();
        let mut workers = vec![Worker::new(); 5];
        workers[0].work(0, start);
        deps.remove(&start);

        for time in 0.. {
            let output = workers.iter_mut().filter_map(|w| w.output(time));

            // Remove finished tasks from dependencies
            for o in output {
                for d in deps.values_mut() {
                    d.remove(&o);
                }
            }

            for w in workers.iter_mut().filter(|w| w.avail(time)) {
                // We end up sorting every time but whatever
                let next = deps
                    .iter()
                    .filter(|kv| kv.1.is_empty()) // Get nodes with no dependencies left
                    .min_by_key(|kv| kv.0) // Sort by alphabetical
                    .map(|kv| *kv.0);

                if let Some(id) = next {
                    deps.remove(&id);
                    w.work(time, id);
                } else {
                    break;
                }
            }

            if deps.is_empty() && workers.iter().all(|w| w.avail(time)) {
                // All tasks done, no more tasks in queue
                println!("Part 2, Finished in {} seconds", time);
                break;
            }
        }
    }
}
