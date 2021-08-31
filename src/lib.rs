use std::str::FromStr;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
struct Model {
    state: State,
    link: ComponentLink<Self>,
}

struct State {
    tasks: Vec<Task>,
    new_task_name: String,
    new_task_assignee: String,
    new_task_mandays: u32,
}

impl State {
    fn increase_status(&mut self, idx: usize) {
        self.tasks
            .get_mut(idx)
            .filter(|e| e.status < 3)
            .map(|e| e.status = e.status + 1);
    }
    fn decrease_status(&mut self, idx: usize) {
        self.tasks
            .get_mut(idx)
            .filter(|e| e.status > 1)
            .map(|e| e.status = e.status - 1);
    }

    fn add_new_task(&mut self, name: String, assignee: String, mandays: u32) {
        self.tasks.push(Task {
            name,
            assignee,
            mandays,
            status: 1,
        });
    }
}
struct Task {
    name: String,
    assignee: String,
    mandays: u32,
    status: u32,
}

enum Msg {
    IncreaseStatus(usize),
    DecreaseStatus(usize),
    UpdateNewTaskName(String),
    UpdateNewTaskAssignee(yew::html::ChangeData),
    UpdateNewTaskMandays(String),
    NewTask,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            state: State {
                tasks: vec![
                    Task {
                        name: "Task 1".to_string(),
                        assignee: "🐱".to_string(),
                        mandays: 3,
                        status: 1,
                    },
                    Task {
                        name: "Task 2".to_string(),
                        assignee: "🐶".to_string(),
                        mandays: 2,
                        status: 1,
                    },
                    Task {
                        name: "Task 3".to_string(),
                        assignee: "🐱".to_string(),
                        mandays: 1,
                        status: 2,
                    },
                    Task {
                        name: "Task 4".to_string(),
                        assignee: "🐹".to_string(),
                        mandays: 3,
                        status: 3,
                    },
                ],
                new_task_name: "".to_string(),
                new_task_assignee: "".to_string(),
                new_task_mandays: 0,
            },
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateNewTaskName(val) => {
                self.state.new_task_name = val;
            }
            Msg::UpdateNewTaskAssignee(val) => {
                if let yew::html::ChangeData::Select(v) = &val {
                    self.state.new_task_assignee = v.value();
                }
            }
            Msg::UpdateNewTaskMandays(val) => {
                if let Ok(v) = u32::from_str(&val) {
                    self.state.new_task_mandays = v;
                }
            }
            Msg::NewTask => {
                self.state.add_new_task(
                    self.state.new_task_name.clone(),
                    self.state.new_task_assignee.clone(),
                    self.state.new_task_mandays,
                );
            }
            Msg::IncreaseStatus(idx) => {
                self.state.increase_status(idx);
            }
            Msg::DecreaseStatus(idx) => {
                self.state.decrease_status(idx);
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            { Self::view_header(&self.state) }
            <div class="columns">
                { Self::view_column(&self, 1, "未対応") } { Self::view_column(&self, 2, "処理中") } { Self::view_column(&self, 3, "完了") }
            </div>
            </>

        }
    }
}

impl Model {
    fn view_column(&self, status: u32, status_text: &str) -> Html {
        let class_name = format!("status-{}", status);

        html! {<div class={class_name + " " + "column"}>
                    { status_text } { self.state.tasks.iter().filter(|e| e.status == status).count() }
        { for self.state.tasks.iter().enumerate().filter(|e| e.1.status == status).map(|e| Self::view_task(e,&self.link)) }
                </div>
                }
    }

    fn view_task((idx, task): (usize, &Task), link: &ComponentLink<Self>) -> Html {
        html! {
            <div>
                { &task.name }
                { &task.assignee }
                { format!("{} 人日", &task.mandays) }
                <button onclick={link.callback(move|_| Msg::DecreaseStatus(idx))}>{ "◀︎" }</button>
                <button onclick={link.callback(move|_| Msg::IncreaseStatus(idx))}>{ "▶︎︎" }</button>
            </div>
        }
    }

    fn view_header(state: &State) -> Html {
        html! {
            <div class="header">
            {&state.new_task_name}
            { "🐱" }
            {&state.new_task_mandays}
            { "追加" }
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
