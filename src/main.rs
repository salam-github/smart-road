use std::collections::VecDeque;

enum Direction {
    North,
    South,
    East,
    West,
}

enum LaneType {
    Right,
    Straight,
    Left,
}

struct Vehicle {
    id: u32,
    direction: Direction,
    lane: LaneType,
    speed: f64,
}

struct Intersection {
    north: VecDeque<Vehicle>,
    south: VecDeque<Vehicle>,
    east: VecDeque<Vehicle>,
    west: VecDeque<Vehicle>,
}

impl Intersection {
    fn new() -> Self {
        Intersection {
            north: VecDeque::new(),
            south: VecDeque::new(),
            east: VecDeque::new(),
            west: VecDeque::new(),
        }
    }

    fn manage_traffic(&mut self) {
        // A simple algorithm to manage traffic
        if let Some(north_vehicle) = self.north.pop_front() {
            println!("Vehicle {} from North passed the intersection", north_vehicle.id);
        }

        if let Some(south_vehicle) = self.south.pop_front() {
            println!("Vehicle {} from South passed the intersection", south_vehicle.id);
        }

        if let Some(east_vehicle) = self.east.pop_front() {
            println!("Vehicle {} from East passed the intersection", east_vehicle.id);
        }
        if let Some(west_vehicle) = self.west.pop_front() {
            println!("Vehicle {} from West passed the intersection", west_vehicle.id);
        }
    }
}

fn main() {
    let mut intersection = Intersection::new();

    // Simulate adding vehicles to the intersection
    intersection.north.push_back(Vehicle { id: 1, direction: Direction::North, lane: LaneType::Right, speed: 50.0 });
    intersection.south.push_back(Vehicle { id: 2, direction: Direction::South, lane: LaneType::Straight, speed: 60.0 });

    // Run the simulation loop
    loop {
        intersection.manage_traffic();

        // Here add more logic for generating vehicles, collecting statistics, etc.
        // For this, we'll just break the loop to prevent it from running infinitely.
        break;
    }
}
