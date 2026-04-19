use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(PartialEq, Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
    size: usize,
}

impl Point {
    fn new(x: usize, y: usize, size: usize) -> Point {
        debug_assert!(x <= size, "x = {x}");
        debug_assert!(y <= size, "y = {y}");
        Point { x, y, size }
    }

    fn index(&self) -> usize {
        self.x * (self.size + 1) + self.y
    }

    fn right(&self) -> Option<Point> {
        if self.x < self.size {
            Some(Point::new(self.x + 1, self.y, self.size))
        } else {
            None
        }
    }

    fn left(&self) -> Option<Point> {
        if self.x > 0 {
            Some(Point::new(self.x - 1, self.y, self.size))
        } else {
            None
        }
    }

    fn up(&self) -> Option<Point> {
        if self.y < self.size {
            Some(Point::new(self.x, self.y + 1, self.size))
        } else {
            None
        }
    }

    fn down(&self) -> Option<Point> {
        if self.y > 0 {
            Some(Point::new(self.x, self.y - 1, self.size))
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
struct Route {
    visited: Vec<bool>,
    last: Point,
}

impl Route {
    fn new(size: usize) -> Route {
        let mut visited = vec![false; (size + 1) * (size + 1)];
        visited[Point::new(0, 0, size).index()] = true;
        Route {
            visited,
            last: Point::new(0, 0, size),
        }
    }

    fn walk(&self, point_arg: Option<Point>) -> Option<Point> {
        if let Some(point) = point_arg {
            if !self.visited[point.index()] {
                return Some(point);
            }
        }
        None
    }

    fn walk_up(&self) -> Option<Point> {
        self.walk(self.last.up())
    }

    fn walk_down(&self) -> Option<Point> {
        self.walk(self.last.down())
    }

    fn walk_right(&self) -> Option<Point> {
        self.walk(self.last.right())
    }

    fn walk_left(&self) -> Option<Point> {
        self.walk(self.last.left())
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let size: usize = args
        .get(1)
        .and_then(|s| s.parse().ok())
        .expect("Usage: <size>");

    let start_time = std::time::Instant::now();
    let result = Arc::new(AtomicU64::new(0));
    let r_clone = Arc::clone(&result);

    // Ctrl+C ハンドラ
    ctrlc::set_handler(move || {
        let current_count = r_clone.load(Ordering::Relaxed);
        let elapsed_sec = start_time.elapsed().as_secs();
        println!(
            "{size} x {size} の現在の進捗: {current_count} とおり. わっしょいわっしょい. 経過時間: {elapsed_sec} 秒"
        );
    })
    .expect("Error setting Ctrl-C handler");

    let mut routes_stack = vec![Route::new(size)];
    let goal = Point::new(size, size, size);

    println!("{size} x {size} の計算を開始するわよ...(Ctrl+C で進捗表示)");

    loop {
        if routes_stack.is_empty() {
            break;
        }

        let route = routes_stack.pop().unwrap();

        if route.last == goal {
            result.fetch_add(1, Ordering::Relaxed);
            continue;
        }

        let mut walk = |point: Point| {
            let mut new_route = route.clone();
            new_route.last = point;
            new_route.visited[point.index()] = true;
            routes_stack.push(new_route);
        };

        if let Some(point) = route.walk_right() {
            walk(point);
        }
        if let Some(point) = route.walk_left() {
            walk(point);
        }
        if let Some(point) = route.walk_up() {
            walk(point);
        }
        if let Some(point) = route.walk_down() {
            walk(point);
        }
    }

    let end_time = std::time::Instant::now();
    let elapsed = end_time.duration_since(start_time);
    let final_count = result.load(Ordering::Relaxed);

    println!("{size} x {size} のときは {final_count} とおりね！");
    println!(
        "所要時間: {} ミリ秒 ({} 秒)",
        elapsed.as_millis(),
        elapsed.as_secs()
    );
}
