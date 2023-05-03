#[derive(PartialEq, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn walk_right(&self) -> Point {
        Point::new(self.x + 1, self.y)
    }

    fn walk_left(&self) -> Point {
        Point::new(self.x - 1, self.y)
    }

    fn walk_up(&self) -> Point {
        Point::new(self.x, self.y + 1)
    }

    fn walk_down(&self) -> Point {
        Point::new(self.x, self.y - 1)
    }
}

#[derive(Clone, Debug)]
struct Route {
    history: Vec<Point>,
    size: i32,
}

impl Route {
    fn new(size: i32) -> Route {
        Route {
            history: vec![Point::new(0, 0)],
            size,
        }
    }

    fn now(&self) -> &Point {
        self.history.last().unwrap()
    }

    fn can_walk_up(&self) -> bool {
        let now = self.now();
        now.y != self.size && !self.history.contains(&now.walk_up())
    }

    fn can_walk_down(&self) -> bool {
        let now = self.now();
        now.y != 0 && !self.history.contains(&now.walk_down())
    }

    fn can_walk_right(&self) -> bool {
        let now = self.now();
        now.x != self.size && !self.history.contains(&now.walk_right())
    }

    fn can_walk_left(&self) -> bool {
        let now = self.now();
        now.x != 0 && !self.history.contains(&now.walk_left())
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let size: i32 = args[1].parse().expect("arg[1] parse error.");

    let mut result = 0u64;

    let mut routes_stack = vec![Route::new(size)];

    loop {
        if routes_stack.is_empty() {
            break;
        }

        let route = routes_stack.pop().unwrap();

        if route.now() == &(Point { x: size, y: size }) {
            result += 1;
            continue;
        }

        if route.can_walk_right() {
            let mut new_route = route.clone();
            new_route.history.push(new_route.now().walk_right());
            routes_stack.push(new_route);
        }

        if route.can_walk_left() {
            let mut new_route = route.clone();
            new_route.history.push(new_route.now().walk_left());
            routes_stack.push(new_route);
        }

        if route.can_walk_up() {
            let mut new_route = route.clone();
            new_route.history.push(new_route.now().walk_up());
            routes_stack.push(new_route);
        }

        if route.can_walk_down() {
            let mut new_route = route.clone();
            new_route.history.push(new_route.now().walk_down());
            routes_stack.push(new_route);
        }
    }

    println!("result = {result}");
}
