use yew::{html, props, Component, Context, Html, Properties};
use yew::prelude::*;

pub enum Msg{
    Clicked(u8)
}

pub struct OptionList{
    pub selected: u8,
}

#[derive(Properties, PartialEq)]
pub struct Props{
    pub options: Vec<String>,
}

impl Component for OptionList{

    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self{
            selected: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            Msg::Clicked(n) => {
                self.selected = n;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html{

        let options = ctx.props().options.clone();
        html!{
            <table>
                {
                for options
                    .iter()
                    .enumerate()
                    .map(|(i, option)| {
                        html!{
                            <tr>
                                <td>
                                    <button onclick={ctx.link().callback(move |_| Msg::Clicked(i as u8))}>
                                        {option}
                                    </button>
                                </td>
                                if i == self.selected as usize{
                                    <td>{"X"}</td>
                                }
                            </tr>
                        }
                    })
                }
            </table>
        }
    }
}
