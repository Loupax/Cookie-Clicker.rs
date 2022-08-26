use console_engine::{MouseButton, KeyCode};
use std::time::{SystemTime, Duration};
fn main() {
    let mut engine = console_engine::ConsoleEngine::init(20,10,3).unwrap();
    let mut state = GameState{
        clicks: 0,
        last_update: SystemTime::now(),
    };

    // A cursor produces 0.1 cps, 1 per 10 seconds
    let mut cursor = CookieProducer{
        name: "Cursor",
        interval: Duration::new(10, 0),
        last_production: SystemTime::now(),
    };


    loop {
        state.last_update = SystemTime::now();
        engine.wait_frame();
        engine.clear_screen();


        let mouse_pos = engine.get_mouse_press(MouseButton::Left);
        if let Some(_) = mouse_pos {
            state.clicks += 1;
        }

        state.clicks += cursor.produce(&state.last_update);
        engine.print(0,0, format!("Cookies: {}", state.clicks).as_str());
        engine.draw();

        if engine.is_key_pressed(KeyCode::Char('q')){
            break;
        }
    }
    println!("Thanks for playing!");
}

struct CookieProducer<'a>{
    name: &'a str,
    interval: Duration,
    last_production: SystemTime,
}
impl CookieProducer<'_>{
    fn produce(&mut self, when: &SystemTime)->i32{
       // how many times has the interval passed since 'when'
        let passedTime = match when.duration_since(self.last_production){
            Ok(n)  => n,
            Err(_) => panic!("Time travel")
        };

        if passedTime > self.interval{
            self.last_production = when.clone();
            return (self.interval.as_secs() / passedTime.as_secs()).try_into().unwrap();
        }
        return 0;
    }
}
struct GameState{
    clicks: i32,
    last_update: SystemTime
}

