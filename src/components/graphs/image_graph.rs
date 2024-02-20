use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;


pub enum ImageGraphMsg {
    Redraw,
    Nothing
}

#[derive(PartialEq, Properties)]
pub struct ImageGraphProps {
    pub data: rustfits::data::array::ArrayData,
}

pub struct ImageGraph {
    canvas: NodeRef,
}

impl Component for ImageGraph{

    type Message = ImageGraphMsg;
    type Properties = ImageGraphProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(ImageGraphMsg::Redraw);
        ImageGraph {
            canvas: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let data = ctx.props().data.clone();
        let image = data.format_data().into_dimensionality::<ndarray::Ix2>().unwrap();
        match msg {
            ImageGraphMsg::Redraw => {
                let element : HtmlCanvasElement = self.canvas.cast().unwrap();
                
                let rect = element.get_bounding_client_rect();
                element.set_height(rect.height() as u32);
                element.set_width(rect.width() as u32);
          
                let backend = CanvasBackend::with_canvas_object(element).unwrap();
                let drawing_area = backend.into_drawing_area();
                drawing_area.fill(&WHITE).unwrap();
                let mut chart = ChartBuilder::on(&drawing_area)
                    // .caption("y=x^2", ("sans-serif", 14).into_font())
                    .margin(5)
                    .x_label_area_size(10)
                    .y_label_area_size(10)
                    .build_cartesian_2d(-2.1f64..0.6f64, -1.2f64..1.2f64).unwrap();
                chart
                .configure_mesh()
                .disable_x_mesh()
                .disable_y_mesh()
                .draw().unwrap();

                let plotting_area = chart.plotting_area();

                let range = plotting_area.get_pixel_range();

                let (pw, ph) = (range.0.end - range.0.start, range.1.end - range.1.start);

                for (x, y) in (0..pw).zip(0..ph) {
                    let value: f64 = image[[x as usize, y as usize]].to_f64();
                    let color = MandelbrotHSL::get_color(value);
                    log::info!("Color: {:?}", color);
                    plotting_area.draw_pixel((x.into(), y.into()), &color).unwrap();
                }

                false
            },
            ImageGraphMsg::Nothing => {true},
          }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <canvas ref={self.canvas.clone()} />
            </div>
        }
    }
}