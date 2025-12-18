use std::{fs, ops::Sub};

use log::debug;
use num_integer::Roots;

#[cfg(test)]
mod tests;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Vector3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Vector3 {
    fn distance(self, other: Self) -> i64 {
        let diff = self - other;
        let x = diff.x.pow(2);
        let y = diff.y.pow(2);
        let z = diff.z.pow(2);
        (x + y + z).sqrt().abs()
    }
}

#[derive(Debug, Copy, Clone)]
struct JunctionBox {
    vector: Vector3,
    circuit_id: Option<i64>,
}

pub(crate) async fn day8(data: Option<String>, iterations: usize) -> i64 {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day8/data/main.txt").unwrap());

    let mut junction_boxes: Vec<JunctionBox> = Vec::new();
    let mut new_circuit_id = 0;

    for line in data.lines() {
        let mut coords = line.split(',');
        let vector = Vector3 {
            x: coords.next().unwrap().parse().unwrap(),
            y: coords.next().unwrap().parse().unwrap(),
            z: coords.next().unwrap().parse().unwrap(),
        };

        junction_boxes.push(JunctionBox {
            vector,
            circuit_id: None,
        });
    }

    let jb_len = junction_boxes.len();
    let mut distances: Vec<(i64, Vector3, Vector3)> = Vec::new();
    for i in 0..jb_len {
        for x in i + 1..jb_len {
            let vec1 = junction_boxes[i].vector;
            let vec2 = junction_boxes[x].vector;
            distances.push((vec1.distance(vec2), vec1, vec2));
        }
    }
    distances.sort_by_key(|x| x.0);
    for distance in distances.iter() {
        debug!("Distance: {:?}", distance);
    }

    let mut i = 0;
    let mut circuit_count = 0;
    let mut part2 = 0;
    while i < iterations || iterations == 0 {
        let distance = distances[i];
        let mut jb_iter_mut = junction_boxes.iter_mut();
        let jb1 = jb_iter_mut.find(|x| x.vector == distance.1).unwrap();
        let jb2 = jb_iter_mut.find(|x| x.vector == distance.2).unwrap();
        // for part 2 - to avoid multiple borrows
        let jb1x = jb1.vector.x;
        let jb2x = jb2.vector.x;

        if jb1.circuit_id.is_none() && jb2.circuit_id.is_none() {
            jb1.circuit_id = Some(new_circuit_id);
            jb2.circuit_id = Some(new_circuit_id);
            new_circuit_id += 1;
            circuit_count += 1;
            debug!("Combined {:?} and {:?} with new ID", jb1, jb2);
        } else if jb1.circuit_id.is_none() && jb2.circuit_id.is_some() {
            jb1.circuit_id = jb2.circuit_id;
            debug!("Adding {:?} to existing circuit of {:?}", jb1, jb2);
        } else if jb1.circuit_id.is_some() && jb2.circuit_id.is_none() {
            jb2.circuit_id = jb1.circuit_id;
            debug!("Adding {:?} to existing circuit of {:?}", jb2, jb1);
        } else if jb1.circuit_id != jb2.circuit_id {
            debug!("Combining circuits for {:?} and {:?}", jb1, jb2);
            let old_circuit = jb2.circuit_id;
            let new_circuit = jb1.circuit_id;

            for item in junction_boxes
                .iter_mut()
                .filter(|x| x.circuit_id == old_circuit)
            {
                debug!(
                    "Remapped {:?} from circuit {:?} to {:?}",
                    item, item.circuit_id, new_circuit
                );
                item.circuit_id = new_circuit;
            }
            circuit_count -= 1;
        }

        if circuit_count == 1 && i > 1
            && junction_boxes.iter().all(|x| x.circuit_id.is_some()) {
                part2 = jb1x * jb2x;
                break;
            }
        i += 1;
    }

    let individual_circuits = junction_boxes
        .iter()
        .filter(|x| x.circuit_id.is_none())
        .count();
    debug!("Individual Circuits: {}", individual_circuits);
    debug!("Circuit ID Count: {}", new_circuit_id);

    let mut counts: Vec<i64> = Vec::new();
    for i in 0..new_circuit_id {
        let count = junction_boxes
            .iter()
            .filter(|x| x.circuit_id == Some(i))
            .count();
        counts.push(count as i64);
        debug!("Count {}: {}", i, count);
        for item in junction_boxes.iter().filter(|x| x.circuit_id == Some(i)) {
            debug!("{:?}", item);
        }
    }

    counts.sort();
    counts.reverse();
    let mut total: i64 = 1;
    for i in 0..3 {
        total *= counts[i];
    }

    if iterations == 0 {
        part2
    } else {
        total
    }
}
