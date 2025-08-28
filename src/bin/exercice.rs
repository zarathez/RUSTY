#[derive(Debug, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
    Critical{ deadline_hours : u32},

}

#[derive(Debug, PartialEq)]
enum Status  {
    Todo,
    InProgress{completion_percent :u32},
    Done,
    Cancelled(String),

}

#[derive(Debug)]
struct Task {
    title : String,
    description :  String,
    priority : Priority,
    status : Status,
    estimated_hours : f32,
}

impl Task {
    fn new(title :String , description:String ) -> Self {
        Self{
            title,
            description,
            priority: Priority::Low ,
            status: Status::Todo ,
            estimated_hours: 1.0
        }
    }

    fn update_status(&mut self , new_status : Status) -> () {
        self.status = new_status;   
    }

    fn is_urgent(&self) -> bool {
        let priority = &self.priority;
        match priority {
            Priority::High | Priority::Critical{..} => true,
            _ => false,
        }
    }

    fn days_needed(&self) -> f32 {
        self.estimated_hours / 8.0
    } 
}

#[derive(Debug)]
struct TaskManager {
    tasks: Vec<Task>,
    owner: String,
}

impl TaskManager {
    fn new(owner_name:String) -> Self {
        Self{
            tasks: Vec::new(), 
            owner:owner_name,
        }
    }

    fn add_task(&mut self , new_task:Task) -> Result<(),String> {
        let v = &self.tasks;
        for task in v.iter() {
            if new_task.title == task.title {
                return Err(format!("task with title {} already exixts" , task.title)); // equivalnt to `return Err();`
            }
        }
        self.tasks.push(new_task);
        Ok(())
    }

    fn find_task(&self , title:&str) -> Option<&Task> {
        for task in self.tasks.iter() {
            if task.title == title {
                return Some(task);
            }
        }
        None
    }

    fn get_urgent_tasks(&self) -> Vec<&Task> {
        let mut urgent_tasks: Vec<&Task> = Vec::new();
        for task in self.tasks.iter() {
            if task.is_urgent() {
                urgent_tasks.push(task);
            }
        }

        urgent_tasks
    }

    fn status_report(&self) -> String {

        let mut to_do_count = 0;
        let mut in_progress_count = 0;
        let mut done_count = 0;
        let mut cancelled_count = 0;

        // Count of each status type (use match and loops)
        for task in self.tasks.iter() {
            match &task.status {
                Status::Todo => to_do_count +=1,
                Status::InProgress{..} => in_progress_count +=1,
                Status::Done => done_count +=1,
                Status::Cancelled(whatever) => cancelled_count+=1,
            }
        }
        

        format!(
            "Owner : {}\nTotal tasks: {}\nList of urgent tasks: {:?}\nTodo: {}\nIn Progress: {}\nDone: {}\nCancelled: {}",
            self.owner,
            self.tasks.len(),
            self.get_urgent_tasks(),
            to_do_count,
            in_progress_count,
            done_count,
            cancelled_count,

        )


    }


}




fn main() {
    let mut manager = TaskManager::new();
    let task1 = Task::new("task1".to_string() , "description1".to_string());
    task1.priority = Priority::High;
    task.status =...

}