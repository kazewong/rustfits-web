use crate::components::views::table_view::{TableView, Tables};
use crate::components::graphs::line_graph::LineGraph;
use crate::components::graphs::image_graph::ImageGraph;
use log::info;
use yew::{html, props, Component, Context, Html, Properties};

pub enum Msg {
    Clicked(u8),
}

pub struct FITSView {
    pub selected: u8,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub file: rustfits::fits::FITS,
}

impl Component for FITSView {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        info!("FITSView created");
        Self { selected: 0 }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked(n) => {
                self.selected = n;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let file = ctx.props().file.clone();
        let options = file.list_headers();
        let header_type = file.hdus[self.selected as usize].header.get_header_type();
        html! {
            <>
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
            <table>
            <tr>
                <th>{"Keyword"}</th>
                <th>{"Value"}</th>
            </tr>
            </table>
            <table class="table-auto block overflow-auto max-h-64 max-w-96">
            {for file.hdus[self.selected as usize].header.list_keywords(true).iter().map(|(key, value)| {
                html! {
                    <tr>
                        <td>{key}</td>
                        <td>{value}</td>
                    </tr>
                }
            })}
            </table>
            if header_type == rustfits::header::HeaderType::ASCIITable || header_type == rustfits::header::HeaderType::BinaryTable{
            <TableView data={
                match &file.hdus[self.selected as usize].data{
                    rustfits::data::data::Data::ASCIITable(table) => {
                        Tables::ASCII(table.format_data())
                    },
                    rustfits::data::data::Data::BinaryTable(table) => {
                        Tables::Binary(table.format_data())
                    },
                    _ => {
                        panic!("Not an Table");
                    }
                }
            } />
            <LineGraph />
            }
            if header_type == rustfits::header::HeaderType::Image{
            <ImageGraph data={
                match &file.hdus[self.selected as usize].data{
                    rustfits::data::data::Data::Array(array) => {
                        array.clone()
                    },
                    _ => {
                        panic!("Not an Image");
                    }
                }
            }/>
            }
            </>
        }
    }
}
