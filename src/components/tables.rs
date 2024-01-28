use yew::{html, props, Component, Context, Html, Properties};
use yew::prelude::*;


pub struct OptionList;

#[derive(Properties, PartialEq)]
pub struct Props{
    pub options: Vec<String>,
    pub selected: u8
}

impl Component for OptionList{

    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            _ => false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html{

        let options = ctx.props().options.clone();
        html!{
            <table>
                {
                for options
                }
            </table>
        }
    }
}
