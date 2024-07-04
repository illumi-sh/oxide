use oxide_common::task::Task;

pub struct NpmRunScriptTask {
    name: String,
    command: String,
}

impl Task for NpmRunScriptTask {
    fn execute(self) {
        println!("Dummy NpmRunScriptTask: [name={}, command={}]", self.name, self.command);
        todo!("run command");
    }

    fn dependencies(self) -> Vec<Box<dyn Task>> {
        vec![]
    }
}