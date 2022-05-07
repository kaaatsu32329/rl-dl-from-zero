use iced::*;

#[derive(Debug, Default)]
struct AgentData {
    title: String,
    reward_map: [[f64; 4]; 3],
    goal_state: (i32, i32),
    wall_state: (i32, i32),
    ys: usize,
    xs: usize,
}

#[derive(Debug, Clone)]
enum Message {}

impl Sandbox for AgentData {
    type Message = Message;

    fn new(
        title: String,
        reward_map: [[f64; 4]; 3],
        goal_state: (i32, i32),
        wall_state: (i32, i32),
    ) -> (Self, Command<Message>) {
        (
            Self {
                title: title,
                reward_map: reward_map,
                goal_state: goal_state,
                wall_state: wall_state,
                ys: reward_map.len(),
                xs: reward_map[0].len(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        self.title
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            _ => {
                None;
            }
        };
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let content = Column::new();

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::Container)
            .into()
    }
}

fn main() -> iced::Result {
    AgentData::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
