## Exercise 1: Personal Task Manager

**Your Mission:** Build a complete task management system from scratch.

**Requirements:**

1. **Create the Enums:**
   - Create a `Priority` enum with variants: `Low`, `Medium`, `High`, and `Critical` (Critical should contain `deadline_hours: u32`)
   - Create a `Status` enum with variants: `Todo`, `InProgress` (containing `completion_percent: u32`), `Done`, and `Cancelled` (containing a String reason)
   - Both enums should derive `Debug` and `PartialEq`

2. **Create the Task Struct:**
   - Fields: `title` (String), `description` (String), `priority` (Priority), `status` (Status), `estimated_hours` (f32)
   - Derive `Debug`

3. **Implement Task Methods:**
   - `new()`: Associated function that creates a task with given title and description. Set default priority to Low, status to Todo, and estimated_hours to 1.0
   - `update_status()`: Method that takes `&mut self` and new status, updates it
   - `is_urgent()`: Returns true if priority is High or Critical (use match)
   - `days_needed()`: Calculates how many 8-hour workdays are needed (estimated_hours / 8.0)

4. **Create the TaskManager Struct:**
   - Fields: `tasks` (Vec<Task>), `owner` (String)
   - Derive `Debug`

5. **Implement TaskManager Methods:**
   - `new()`: Associated function that creates an empty manager with given owner name
   - `add_task()`: Returns `Result<(), String>`. Loop through existing tasks, return `Err` if title already exists, otherwise push and return `Ok(())`
   - `find_task()`: Takes a title string slice, returns `Option<&Task>`. Use a for loop to search
   - `get_urgent_tasks()`: Returns `Vec<&Task>` containing all urgent tasks
   - `status_report()`: Returns a formatted String showing:
     - Owner name
     - Total tasks count
     - Count of each status type (use match and loops)
     - List of urgent task titles

6. **In main(), demonstrate everything:**
   - Create a TaskManager
   - Create 2 tasks: one urgent (High or Critical priority) and one normal (Low priority)
   - Add them to the manager (handle the Result with match)
   - Update one task's status to InProgress
   - Find a task by title (handle the Option with match)
   - Print all urgent tasks
   - Generate and print the status report
   - Test if `Priority::High == Priority::High` prints true

---

