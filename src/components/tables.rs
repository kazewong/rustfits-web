use yew::{Component, Context, html, Html, Properties};

pub struct OptionList{
    pub options: Vec<String>,
    pub selected: u8
}

impl Component for OptionList{

    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self{
            options: Vec::new(),
            selected: 0
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            _ => false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html{
        let options = self.options.iter().enumerate().map(|(i, option)| {
            let selected = if i == self.selected as usize{
                "selected"
            }else{
                ""
            };
            html!{
                <p>{option}</p>
            }
        });
        html!{
            <select>
                {for options}
            </select>
        }
    }
}
