use gloo_console::log;
use plotters::coord::Shift;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ScoreChartProps {
    pub avgs: Vec<f32>,
}

pub struct ScoreChart {
    canvas_ref: NodeRef,
    chart: Option<DrawingArea<CanvasBackend, Shift>>,
}

impl ScoreChart {
    fn draw_grid(&self) {
        if let Some(chart) = &self.chart {
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
    }

    fn draw_avgs(&self, avgs: &Vec<f32>) {
        if let Some(chart) = &self.chart {
            let color = RGBColor(97, 97, 97);
            if avgs.len() > 1 {
                let x_len = (avgs.len() - 1) as f64;
                let values = || {
                    avgs
                        .iter()
                        .enumerate()
                        .map(|(x, y)| (x as f64, *y as f64))
                };
                ChartBuilder::on(chart)
                    .build_cartesian_2d(0.0..x_len, 0.0..5.0)
                    .unwrap()
                    .draw_series(
                        LineSeries::new(
                            values(),
                            color,
                        )
                    )
                    .unwrap();
            } else if avgs.len() == 1 {
                // Draw a single value as circle
                let dot = EmptyElement::at((0.0, avgs[0] as f64))
                    + Circle::new((0, 0), 4, ShapeStyle::from(color).filled());
                ChartBuilder::on(chart)
                    .build_cartesian_2d(0.0..0.0, 0.0..5.0)
                    .unwrap()
                    .plotting_area()
                    .draw(&dot);
            }
        }
    }
}

impl Component for ScoreChart {
    type Message = ();
    type Properties = ScoreChartProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            canvas_ref: NodeRef::default(),
            chart: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <canvas ref={self.canvas_ref.clone()} />
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let element : HtmlCanvasElement = self.canvas_ref.cast().unwrap();
        // Set canvas size
        let rect = element.get_bounding_client_rect();
        element.set_height(rect.height() as u32);
        element.set_width(rect.width() as u32);

        let backend = CanvasBackend::with_canvas_object(element).unwrap();
        self.chart = Some(backend.into_drawing_area());

        self.draw_grid();
        self.draw_avgs(&ctx.props().avgs);
    }
}
