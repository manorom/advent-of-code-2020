
#[derive(Debug, Clone, Copy, PartialEq)]
enum ActionKind {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Action {
    kind: ActionKind,
    value: i64,
}

impl From<&str> for Action {
    fn from(s: &str) -> Action {
        use ActionKind::*;

        let kind = match s.chars().next() {
            Some('N') => North,
            Some('S') => South,
            Some('E') => East,
            Some('W') => West,
            Some('L') => Left,
            Some('R') => Right,
            Some('F') => Forward,
            _   => panic!("Unknown action"),
        };

        let value = s[1..].parse::<i64>().expect("Unknown value for action");

        Action {
            kind,
            value
        }
    }
}

impl Action {
    fn navigate_part1(&self, posx: i64, posy: i64, orientation: i64) -> (i64, i64, i64) {
        return match self.kind {
            ActionKind::North => (posx, posy - self.value, orientation),
            ActionKind::South => (posx, posy + self.value, orientation),
            ActionKind::West =>  (posx - self.value, posy, orientation),
            ActionKind::East => (posx + self.value, posy, orientation),
            ActionKind::Left => {
                (posx, posy,(orientation - self.value).rem_euclid(360))
            },
            ActionKind::Right => (posx, posy, (orientation + self.value).rem_euclid(360)),
            ActionKind::Forward => {
                match orientation {
                    0   => (posx, posy - self.value, orientation),
                    90  => (posx + self.value, posy, orientation),
                    180 => (posx, posy + self.value, orientation),
                    270 => (posx - self.value, posy, orientation),
                    _ => panic!("Unhandled direction {}", orientation),
                }
            },
        };
    }

    fn rotate_relative_clockwise(&self, posx: i64, posy: i64, rotation: i64) -> (i64, i64) {
        match rotation {
            0   => (posx, posy),
            90  => (posy, -posx),
            180 => (-posx, -posy),
            270 => (-posy, posx),
            _   => panic!("Unhandled rotation {}", rotation),
        }
    }

    fn navigate_part2(&self, posx: i64, posy: i64, waypointx: i64, waypointy: i64) -> ((i64, i64), (i64, i64)) {
        return match self.kind {
            ActionKind::North => ((posx, posy), (waypointx, waypointy - self.value)),
            ActionKind::South => ((posx, posy), (waypointx, waypointy + self.value)),
            ActionKind::West =>  ((posx, posy), (waypointx - self.value, waypointy)),
            ActionKind::East =>  ((posx, posy), (waypointx + self.value, waypointy)),
            ActionKind::Left =>  ((posx, posy), self.rotate_relative_clockwise(waypointx, waypointy, self.value)),
            ActionKind::Right => ((posx, posy), self.rotate_relative_clockwise(waypointx, waypointy, (-self.value).rem_euclid(360))),
            ActionKind::Forward => ((posx + waypointx*self.value, posy + waypointy*self.value), (waypointx, waypointy)),
        };
    }
}

fn main() {
    let input = include_str!("input.txt");

    let input = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| Action::from(l))
        .collect::<Vec<_>>();

    let distance_part1 = input
        .iter()
        .fold((0, 0, 90), |accum, a| {
            a.navigate_part1(accum.0, accum.1, accum.2)
        });
    
    println!("Travelled manhatten distance is (part1) {}", distance_part1.0.abs() + distance_part1.1.abs());


    let distance_part2 = input
        .iter()
        .fold(((0, 0), (10, -1)), |((posx, posy), (waypointx, waypointy)), a| {
            //dbg!((posx, posy, waypointx, waypointy));
            a.navigate_part2(posx, posy, waypointx, waypointy)
        });

    //dbg!(&distance_part2);
    println!("Waypoint is set to manhatten distance (part2) of {}", distance_part2.0.0.abs() + distance_part2.0.1.abs());

}
