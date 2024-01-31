
use rustfits::data::tables::{ASCIIField, BinaryField, Matrix2D};
use yew::{html, Component, Context, Html, Properties};
use log::info;
pub struct TableView{
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tables{
    ASCII(Matrix2D<ASCIIField>),
    Binary(Matrix2D<BinaryField>),
}

pub enum Fields{
    ASCII(ASCIIField),
    Binary(BinaryField),
}

impl Tables{
    pub fn get_row(&self, row: usize) -> Vec<Fields>{
        match self{
            Tables::ASCII(table) => {
                table.get_row(row.try_into().unwrap()).iter().map(|x| Fields::ASCII(x.clone())).collect()
            },
            Tables::Binary(table) => {
                table.get_row(row.try_into().unwrap()).iter().map(|x| Fields::Binary(x.clone())).collect()
            },
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props{
    pub data: Tables,
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
            // {row.iter().collec   t::<Html>()}
            </>
        }
    }
}