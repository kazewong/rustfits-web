use rustfits::data::tables::{ASCIIField, ASCIITable, BinaryField, BinaryTable, Matrix2D};
use yew::{html, props, Component, Context, Html, Properties};

pub struct TableView{
}

#[derive(Properties, PartialEq)]
pub struct Props{
    pub data: Matrix2D<ASCIIField>,
}

impl Component for TableView{

    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self{
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        todo!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html{

        let data = ctx.props().data.clone();

        html!{
            <>
            {for data.into_iter().map(|row|{
                html!{
                    <tr>
                        {
                            for row.iter().map(|field|{
                                html!{
                                    <td>{field}</td>
                                }
                            })
                        }
                    </tr>
                }
            })}
            </>
        }
    }
}