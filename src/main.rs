use console_engine::{MouseButton, KeyCode};
use std::time::{SystemTime, Duration};
fn main() {
    let start_time = SystemTime::now();
    let mut engine = console_engine::ConsoleEngine::init(20,10,3).unwrap();
    let mut state = GameState{
        clicks: 0,
        last_update: start_time.clone(),
    };

    // Each cursor produces 0.1 cps, 1 per 10 seconds
    let cursor = CookieProducer{
        name: "Cursor",
        interval: Duration::new(10, 0),
        last_production: start_time.clone(),
        how_many: 1,
    };

    // Each grandma produces one cps
    let grandma = CookieProducer{
        name: "Grandma",
        interval: Duration::new(1,0),
        last_production: start_time.clone(),
        how_many: 100,
    };
    
    let mut producers = CookieProducers{
        producers: &mut[cursor, grandma],
    };

    loop {
        state.last_update = SystemTime::now();
        engine.wait_frame();
        engine.clear_screen();


        let mouse_pos = engine.get_mouse_press(MouseButton::Left);
        if let Some(_) = mouse_pos {
            state.clicks += 1;
        }

        state.clicks += producers.produce(&state.last_update);
        engine.print(0,0, format!("Cookies: {}", state.clicks).as_str());
        engine.draw();

        if engine.is_key_pressed(KeyCode::Char('q')){
            break;
        }
    }
}

struct CookieProducers<'slice>{
    producers: &'slice mut [CookieProducer<'slice>],
}

impl CookieProducers<'_>{
    fn produce(&mut self, when: &SystemTime)->i32{
        let mut total = 0;
        for p in self.producers.iter_mut(){
            total += p.produce(when);
        };
        return total;
    }
}

struct CookieProducer<'a>{
    name: &'a str,
    interval: Duration,
    last_production: SystemTime,
    how_many: i32,
}

impl CookieProducer<'_>{
    fn produce(&mut self, when: &SystemTime)->i32{
       // how many times has the interval passed since 'when'
        let passed_time = match when.duration_since(self.last_production){
            Ok(n)  => n,
            Err(_) => panic!("Time travel")
        };

        if passed_time > self.interval{
            self.last_production = when.clone();
            let p:i32 = (self.interval.as_secs() / passed_time.as_secs()).try_into().unwrap();
            return p * self.how_many;
        }
        return 0;
    }
}

struct GameState{
    clicks: i32,
    last_update: SystemTime
}

