#[derive(Debug , PartialEq)]
enum Event {
    Error { code :u32 , message: String},
    Warning(String),
    Info,
}

struct Logger {
    collection:Vec<Event>
}

impl Logger {

    fn new() -> Self {
        Self {collection : Vec::new()}
    }

    fn log(&mut self , event:Event) -> Result<() ,String> {
        match &event {
            Event::Error{code:999 , .. } => return Err("code 999 not allowed".to_string()),
            _ => {
                self.collection.push(event);
                Ok(())
                
            },
        }
        Ok(())

    }   

    fn count_errors(&self) -> u32 {
        let mut counter:u32 =0;
        for event in self.collection.iter() {
            match event {
                Event::Error{..} => counter += 1,
                _ => (), 
            }
        } 
        counter
    }

    fn find_by_message(&self , msg : &str) -> Option<&Event> {
        for event in self.collection.iter() {
            match event {
                Event::Error{message:msg , ..} => return Some(&event),
                Event::Warning(msg) => return Some(&event),
                _ => (),
            }
        }
        None
    }


}

fn main() {

    let mut logger:Logger = Logger::new();
    let result1:Result<String ,String> =logger.log(Event::Error{code :999 , message:"server error".to_string()});
    let result2:Result<String ,String> =logger.log(Event::Warning("Warning !".to_string()));
    let result3:Result<String ,String> =logger.log(Event::Info);

    println!("{:?}", result1);
    println!("{:?}", result2);
    println!("{:?}", result3);

    let mut finding_result:Option<&Event> = logger.find_by_message("server error");
    println!("{:?}", finding_result);




}   