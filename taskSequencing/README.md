# The Geranium RT Scheduler
This project comes with a tasks scheduling feature, allowing you to simply write your tasks as structures with an execute method and provide a scheduling ; the Geranium RT Scheduler Generator will handle everything, generating a main.rs program that executes your tasks according to the given scheduling.  

## How to use
To enable the scheduling features, simply use the `-s` switch when generating a project using the project generator (gen-new-project.py).  
Your project will be generated with 
- an empty, placeholder main (that should not be modified)
- a user_tasks.rs file, where you will put your tasks (see below)
- a example scheduling JSON file (see below)
- the generate-main.py script

The script takes one argument : a JSON file describing the sheduling you want to use. Once you run it with your scheduling JSON file, it will generate a main.rs file that runs the tasks from user_tasks.rs with the given timing (and then loop back infinitely to the beginning of the scheduling).  

## Making tasks and schedulings

### Tasks
Tasks as defined as structures implementing the Task trait (found in geranium_seq::sequencer::task ; the import is already present in the example user_tasks.rs generated initially).   
Having your tasks be structures may seem a bit odd, but it allows your tasks to very easily hold persistent data. It induces no overehead if you do not keep any data (= no field in the task structure)  

The Task traits has 3 methods to implement : 
- the `execute(&mut self) -> ()` method, which is the actual task execution
- the `new() -> MyTask` static method, which simply creates an instance of the task ; use this to initialize your task properly
- the OPTIONAL `init(&mut self) -> ()` method, which will be called once after all tasks have been constructed. May be useful for initializations that require that other tasks already exist (see Shared Data)

Example task : 
```rust
pub struct LedOn {}
impl Task for LedOn {
    fn execute(&mut self) -> () {
        digital_write(MY_LED, HIGH);
    }

    fn new() -> LedOn {
        LedOn {}
    }
}
```

Keep in mind that you do NOT need to call any of this yourself. You don't even have a main function, as one will be generated for you.  

### Scheduling
Once your tasks are ready, you need to tell the scheduler the timing of their execution.  

It is important to understand the notion of "job" in a task scheduling, and how it differs from the notion of task.  
A job is, simply put, an execution of a task. If, in your scheduling, a same task is run three times, each of these occurences is a separate job, that all run the same task.  
So, when you define a scheduling, what you're actually scheduling are the jobs, as they are the ones that need to have a precise timing, while a task is simply the abstract action that jobs will be running. 

It is also important to note that your tasks/jobs can be scheduled in three ways (called policies)
- "strict" : jobs will be run at the times you specified, period. 
- "deadline" : adds the ability to specify a maximum duration for each task : if a jobs runnning this task takes longer than this duration, the program will safely crash. 
- "preemptive" : jobs now don't need to be run in one go anymore : a long job may be interrupted to run a shorter but more frequent task, and then start again later.  

All times are in ms.  

#### Strict scheduling JSON file

The structure is 
```
{
    "version": "v0.1", <-- leave this unchanged
    "type": "strict",
    "hyperperiod": <the total duration of your scheduling>
    
    "tasks": [
        {
            "id": <id used by jobs to refer to the task>,
            "label": <Display name, used for debugging purposes>, <-- MAY BE OMITTED
            "taskStruct": "Task1"   <-- Name of the struct representing this task
        },
        ...
    ],
    
    "jobs": [
        {
            "taskId": <The id of the task this job is running>,
            "startTime": <Date, i.e. time since the beginning of the scheduling, at which this job will run>
        },
        ...
    ]
}
```

Example : 

```json
{
    "version": "v0.1",
    "type": "strict",
    "hyperperiod": 1000,
    
    "tasks": [
        {
            "id": "led_on",
            "label": "LED On",
            "taskStruct": "LedOn"
        },
        {
            "id": "led_off",
            "label": "LED Off",
            "taskStruct": "LedOff"
        }
    ],
    
    "jobs": [
        {
            "taskId": "led_on",
            "startTime": 0
        },
        {
            "taskId": "led_off",
            "startTime": 500
        }
    ]
}
    
```


#### Deadline scheduling JSON file

The structure is 
```
{
    "version": "v0.1", <-- leave this unchanged
    "type": "deadline",
    "hyperperiod": <the total duration of your scheduling>
    
    "tasks": [
        {
            "id": <id used by jobs to refer to the task>,
            "label": <Display name, used for debugging purposes>, <-- MAY BE OMITTED
            "taskStruct": <Name of the struct representing this task>,
            "duration" : <maximum time this task may run for>
        },
        ...
    ],
    
    "jobs": [
        {
            "taskId": <The id of the task this job is running>,
            "startTime": <Date, i.e. time since the beginning of the scheduling, at which this job will run>
        },
        ...
    ]
}
```

Example : 

```json
{
    "version": "v0.1",
    "type": "strict",
    "hyperperiod": 1000,
    
    "tasks": [
        {
            "id": "led_on",
            "label": "LED On",
            "taskStruct": "LedOn",
            "duration" : 10
        },
        {
            "id": "led_off",
            "label": "LED Off",
            "taskStruct": "LedOff",
            "duration" : 10
        }
    ],
    
    "jobs": [
        {
            "taskId": "led_on",
            "startTime": 0
        },
        {
            "taskId": "led_off",
            "startTime": 500
        }
    ]
}
    
```