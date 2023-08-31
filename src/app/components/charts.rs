use plotters::coord::Shift;
use plotters::prelude::*;
use web_sys::HtmlDivElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ScoreChartProps {
    pub avgs: Vec<f32>,
}

pub struct ScoreChart {
    canvas_ref: NodeRef,
}

impl Component for ScoreChart {
    type Message = ();
    type Properties = ScoreChartProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            canvas_ref: NodeRef::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div ref={self.canvas_ref.clone()}>
                <svg viewBox="0 0 100 100" />
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let mut svg = String::new();
        let div_wrapper: HtmlDivElement = self.canvas_ref.cast().unwrap();
        let rect = div_wrapper.get_bounding_client_rect();

        {
            let backend = SVGBackend::with_string(
                &mut svg,
                (rect.width() as u32, rect.height() as u32)
            );
            let chart = backend.into_drawing_area();

            draw_grid(&chart);
            draw_avgs(&chart, &ctx.props().avgs);
        }

        div_wrapper.set_inner_html(&svg);
    }
}

fn draw_grid(chart: &DrawingArea<SVGBackend<'_>, Shift>) {
    let child_drawing_areas = chart.split_evenly((5, 1));
    for (i, area) in child_drawing_areas.into_iter().enumerate() {
        let alpha = match i {
            0 => 1.0,
            1 => 0.8,
            2 => 0.6,
            3 => 0.4,
            4 => 0.2,
            _ => unreachable!(),
        };
        area.fill(&RGBAColor(223, 171, 74, alpha)).unwrap();
    }
}

fn draw_avgs(chart: &DrawingArea<SVGBackend<'_>, Shift>, avgs: &Vec<f32>) {
    let color = RGBColor(97, 97, 97);
    if avgs.len() > 1 {
        let x_len = (avgs.len() - 1) as f64;
        let values = || {
            avgs
                .iter()
                .enumerate()
                .map(|(x, y)| (x as f64, *y as f64))
        };
        ChartBuilder::on(&chart)
            .build_cartesian_2d(0.0..x_len, 0.0..5.0)
            .unwrap()
            .draw_series(
                LineSeries::new(
                    values(),
                    color.stroke_width(3),
                )
            )
            .unwrap();
    } else if avgs.len() == 1 {
        // Draw a single value as circle
        let dot = EmptyElement::at((0.0, avgs[0] as f64))
            + Circle::new((0, 0), 4, ShapeStyle::from(color).filled());
        let _ = ChartBuilder::on(chart)
            .build_cartesian_2d(0.0..0.0, 0.0..5.0)
            .unwrap()
            .plotting_area()
            .draw(&dot);
    }
}
