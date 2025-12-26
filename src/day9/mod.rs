use crate::core::vector2::Vector2;
use std::fs;
#[cfg(test)]
mod tests;

fn point_on_edge(point: Vector2, p1: Vector2, p2: Vector2) -> bool {
    let Vector2 { x, y } = point;
    let Vector2 { x: x1, y: y1 } = p1;
    let Vector2 { x: x2, y: y2 } = p2;

    // Check if point is within the bounding box of the segment
    let min_x = x1.min(x2);
    let max_x = x1.max(x2);
    let min_y = y1.min(y2);
    let max_y = y1.max(y2);

    if x < min_x || x > max_x || y < min_y || y > max_y {
        return false;
    }

    // Use cross product to check collinearity
    // If cross product is 0, the points are collinear
    let cross_product = (y - y1) * (x2 - x1) - (x - x1) * (y2 - y1);
    cross_product == 0
}

fn point_in_polygon(point: Vector2, polygon: &[Vector2]) -> bool {
    // Check if point is a vertex
    for polygon_point in polygon {
        if point == *polygon_point {
            return true;
        }
    }

    // Check if point lies on any edge
    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        if point_on_edge(point, polygon[j], polygon[i]) {
            return true;
        }
        j = i;
    }

    // Ray casting algorithm for interior points
    let Vector2 { x, y } = point;
    let mut inside = false;

    j = polygon.len() - 1;
    for i in 0..polygon.len() {
        let Vector2 { x: xi, y: yi } = polygon[i];
        let Vector2 { x: xj, y: yj } = polygon[j];

        if (yi > y) != (yj > y) {
            let dy = yj - yi;
            let lhs = (x - xi) * dy;
            let rhs = (xj - xi) * (y - yi);
            if (dy > 0 && lhs < rhs) || (dy < 0 && lhs > rhs) {
                inside = !inside;
            }
        }

        j = i;
    }

    inside
}

fn rect_from_corners(c1: Vector2, c2: Vector2) -> [Vector2; 4] {
    [
        c1,
        c2,
        Vector2 { x: c1.x, y: c2.y },
        Vector2 { x: c2.x, y: c1.y },
    ]
}

// Check if two line segments intersect (not just touch at endpoints)
fn segments_intersect(a1: Vector2, a2: Vector2, b1: Vector2, b2: Vector2) -> bool {
    // Using cross product to determine orientation
    fn cross(o: Vector2, a: Vector2, b: Vector2) -> i64 {
        (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
    }

    let d1 = cross(b1, b2, a1);
    let d2 = cross(b1, b2, a2);
    let d3 = cross(a1, a2, b1);
    let d4 = cross(a1, a2, b2);

    // Check if segments straddle each other
    if ((d1 > 0 && d2 < 0) || (d1 < 0 && d2 > 0)) && ((d3 > 0 && d4 < 0) || (d3 < 0 && d4 > 0)) {
        return true;
    }

    false
}

// Check if entire rectangle is inside polygon
fn rect_in_polygon(rect: &[Vector2; 4], polygon: &[Vector2]) -> bool {
    let c1 = rect[0];
    let c2 = rect[1];

    // If corners share x or y, this is a line/point, not a rectangle
    if c1.x == c2.x || c1.y == c2.y {
        return false;
    }

    // Check all 4 corners are inside/on the polygon
    for corner in rect {
        if !point_in_polygon(*corner, polygon) {
            return false;
        }
    }

    // Define the 4 edges of the rectangle
    let min_x = c1.x.min(c2.x);
    let max_x = c1.x.max(c2.x);
    let min_y = c1.y.min(c2.y);
    let max_y = c1.y.max(c2.y);

    let rect_edges = [
        (Vector2::new(min_x, min_y), Vector2::new(max_x, min_y)), // bottom
        (Vector2::new(max_x, min_y), Vector2::new(max_x, max_y)), // right
        (Vector2::new(max_x, max_y), Vector2::new(min_x, max_y)), // top
        (Vector2::new(min_x, max_y), Vector2::new(min_x, min_y)), // left
    ];

    // Check if any polygon edge intersects any rectangle edge
    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        let poly_edge_start = polygon[j];
        let poly_edge_end = polygon[i];

        for (rect_start, rect_end) in &rect_edges {
            if segments_intersect(poly_edge_start, poly_edge_end, *rect_start, *rect_end) {
                return false;
            }
        }

        j = i;
    }

    true
}

pub(crate) async fn day9(data: Option<String>) -> (i64, i64) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day9/data/main.txt").unwrap());

    let mut vectors: Vec<Vector2> = Vec::new();

    for line in data.lines() {
        let mut split = line.split(',');
        let v = Vector2 {
            x: split.next().unwrap().to_string().parse().unwrap(),
            y: split.next().unwrap().to_string().parse().unwrap(),
        };
        vectors.push(v);
    }
    // Don't sort - vectors should already be in polygon order

    let mut largest: (i64, Vector2, Vector2) = (0, Vector2::new(0, 0), Vector2::new(0, 0));
    let mut largest_in_polygon = largest;

    let vectors_len = vectors.len();
    for (i1, vec1) in vectors[0..vectors_len - 1].iter().enumerate() {
        for vec2 in vectors[i1 + 1..].iter() {
            let area = vec1.area(*vec2);
            if area > largest.0 {
                largest = (area, *vec1, *vec2);
            }
            if area > largest_in_polygon.0 {
                let rect = rect_from_corners(*vec1, *vec2);
                // Check if entire rectangle (all corners and edges) is inside polygon
                if rect_in_polygon(&rect, &vectors) {
                    largest_in_polygon = (area, *vec1, *vec2);
                }
            }
        }
    }

    (largest.0, largest_in_polygon.0)
}
