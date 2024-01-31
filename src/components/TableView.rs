use core::fmt;

use rustfits::data::tables::{ASCIIField, ASCIITable, BinaryField, BinaryTable, Matrix2D};
use web_sys::console;
use yew::{html, props, Component, Context, Html, Properties};
use log::info;
pub struct TableView{
}

#[derive(Debug, Clone, PartialEq)]
enum TableField{
    ASCII(ASCIIField),
    Binary(BinaryField),
}

impl fmt::Display for TableField{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            TableField::ASCII(field) => write!(f, "{}", field),
            TableField::Binary(field) => write!(f, "{}", field),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props{
    pub data: Matrix2D<TableField>,
}

impl Component for TableView{

    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        info!("TableView created");
        Self{
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        todo!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html{

        let data = ctx.props().data.clone();
        let row = data.get_row(0);

        html!{
            <>
            {row.iter().collect::<Html>()}
            </>
        }
    }
}