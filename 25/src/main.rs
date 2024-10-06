use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input(filename: &str) -> std::io::Result<HashMap<String, Vec<String>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut graph = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(": ").collect();
        let component = parts[0].to_string();
        let connections: Vec<String> = parts[1].split_whitespace().map(String::from).collect();

        graph.entry(component.clone()).or_insert_with(Vec::new).extend(connections.clone());
        for conn in connections {
            graph.entry(conn).or_insert_with(Vec::new).push(component.clone());
        }
    }

    Ok(graph)
}


fn find_minimum_cut(graph: &HashMap<String, Vec<String>>) -> (HashSet<String>, HashSet<String>) {
    let mut best_cut = (HashSet::new(), HashSet::new());
    let mut best_cut_size = usize::MAX;

    for _ in 0..100 {  // Run multiple times to increase chances of finding the global minimum
        let (group1, group2) = karger_algorithm(graph);
        let cut_size = count_edges_between(&group1, &group2, graph);

        if cut_size < best_cut_size {
            best_cut = (group1, group2);
            best_cut_size = cut_size;
        }

        if best_cut_size == 3 {
            break;
        }
    }

    best_cut
}


fn karger_algorithm(graph: &HashMap<String, Vec<String>>) -> (HashSet<String>, HashSet<String>) {
    let mut nodes: Vec<HashSet<String>> = graph.keys().map(|k| {
        let mut set = HashSet::new();
        set.insert(k.clone());
        set
    }).collect();

    while nodes.len() > 2 {
        let i = rand::random::<usize>() % nodes.len();
        let j = {
            let edges = nodes[i].iter().flat_map(|n| graph[n].iter()).collect::<Vec<_>>();
            let random_edge = edges[rand::random::<usize>() % edges.len()];
            nodes.iter().position(|set| set.contains(random_edge)).unwrap()
        };

        if i != j {
            let smaller = nodes.remove(std::cmp::max(i, j));
            let larger = nodes.get_mut(std::cmp::min(i, j)).unwrap();
            larger.extend(smaller);
        }
    }

    (nodes[0].clone(), nodes[1].clone())
}

fn count_edges_between(group1: &HashSet<String>, group2: &HashSet<String>, graph: &HashMap<String, Vec<String>>) -> usize {
    group1.iter().flat_map(|n| graph[n].iter())
        .filter(|&n| group2.contains(n))
        .count()
}

fn main() -> std::io::Result<()> {
    let graph = read_input("input.txt")?;
    let (group1, group2) = find_minimum_cut(&graph);
    println!("Group 1: {:?}, Group 2: {:?}", group1.len(), group2.len());
    println!("Result: {}", group1.len() * group2.len());
    Ok(())
}

