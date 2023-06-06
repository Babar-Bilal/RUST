enum Office {
    Arrive, Clean, StartProject, FinishProject, Leave
}

struct DailyTask {
    task: Office,
    progress: String
}

fn print_task(dailytask: DailyTask) {
    match dailytask.task {
        Office::Arrive => println!("Arrive at the office!"),
        Office::Clean => println!("Please clean your desk!"),
        Office::StartProject => println!("Start your project!"),
        Office::FinishProject => println!("Finish your project before deadline!"),
        Office::Leave => print!("Leave for home!")
    }
    println!("{:?}", dailytask.progress);
}

fn main() {
    let arrive = DailyTask {
        task: Office::Arrive,
        progress: "Hello Everyone".to_string(),
    };
    print_task(arrive);  

    let clean = DailyTask {
        task: Office::Clean,
        progress: "I cleaned my desk".to_string(),
    };
    print_task(clean);  

    let start = DailyTask {
        task: Office::StartProject,
        progress: "Reviewing the project".to_string(),
    };
    print_task(start);

    let finish = DailyTask {
        task: Office::FinishProject,
        progress: "Submit the project".to_string(),
    };

    print_task(finish);

    let leave = DailyTask {
        task: Office::Leave,
        progress: "See you all tomorrow".to_string(),
    };
    print_task(leave);
}

