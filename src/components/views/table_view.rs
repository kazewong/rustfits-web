use core::fmt;
use log::info;
use rustfits::data::tables::{ASCIIField, BinaryField, Matrix2D};
use yew::{html, Component, Context, Html, Properties};
pub struct TableView {}

#[derive(Debug, Clone, PartialEq)]
pub enum Tables {
    ASCII(Matrix2D<ASCIIField>),
    Binary(Matrix2D<BinaryField>),
}

pub enum Fields {
    ASCII(ASCIIField),
    Binary(BinaryField),
}

impl fmt::Display for Fields {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fields::ASCII(field) => {
                write!(f, "{}", field)
            }
            Fields::Binary(field) => {
                write!(f, "{}", field)
            }
        }
    }
}

impl Tables {
    pub fn get_row(&self, row: usize) -> Vec<Fields> {
        match self {
            Tables::ASCII(table) => table
                .get_row(row.try_into().unwrap())
                .iter()
                .map(|x| Fields::ASCII(x.clone()))
                .collect(),
            Tables::Binary(table) => table
                .get_row(row.try_into().unwrap())
                .iter()
                .map(|x| Fields::Binary(x.clone()))
                .collect(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data: Tables,
}

impl Component for TableView {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        info!("TableView created");
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        todo!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let data = ctx.props().data.clone();
        let row = data.get_row(0);

        html! {
            <>
            {row.iter().collect::<Html>()}
            </>
        }
    }
}
