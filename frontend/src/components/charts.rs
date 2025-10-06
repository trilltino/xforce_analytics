use yew::prelude::*;
use yew_hooks::prelude::*;
use plotly::{Plot, Scatter, Bar, Pie, Layout, common::{Mode, Marker, Title, Font, Line}, layout::Axis};
use serde_json::Value;

/// Pie Chart Component - For category/distribution visualization
#[derive(Properties, PartialEq)]
pub struct PieChartProps {
    pub id: String,
    pub labels: Vec<String>,
    pub values: Vec<f64>,
    pub title: String,
    #[prop_or_default]
    pub height: Option<usize>,
}

#[function_component(PieChart)]
pub fn pie_chart(props: &PieChartProps) -> Html {
    let id = props.id.clone();
    let labels = props.labels.clone();
    let values = props.values.clone();
    let title = props.title.clone();
    let height = props.height.unwrap_or(400);

    let plot_task = use_async::<_, _, ()>({
        let id = id.clone();
        async move {
            let trace = Pie::new(values)
                .labels(labels)
                .marker(Marker::new().line(Line::new().color("#ffffff").width(2.0)));

            let mut plot = Plot::new();
            plot.add_trace(trace);

            let layout = Layout::new()
                .title(Title::with_text(&title))
                .height(height)
                .paper_background_color("#1a1f36")
                .plot_background_color("#1a1f36")
                .font(Font::new().color("#ffffff").family("Inter"));

            plot.set_layout(layout);

            plotly::bindings::new_plot(&id, &plot).await;
            Ok(())
        }
    });

    use_effect_with((), move |_| {
        plot_task.run();
        || ()
    });

    html! {
        <div id={id} class="plotly-chart"></div>
    }
}

/// Bar Chart Component - For comparisons and distributions
#[derive(Properties, PartialEq)]
pub struct BarChartProps {
    pub id: String,
    pub x_labels: Vec<String>,
    pub y_values: Vec<f64>,
    pub title: String,
    pub x_title: String,
    pub y_title: String,
    #[prop_or_default]
    pub height: Option<usize>,
    #[prop_or_default]
    pub color: Option<String>,
}

#[function_component(BarChart)]
pub fn bar_chart(props: &BarChartProps) -> Html {
    let id = props.id.clone();
    let x_labels = props.x_labels.clone();
    let y_values = props.y_values.clone();
    let title = props.title.clone();
    let x_title = props.x_title.clone();
    let y_title = props.y_title.clone();
    let height = props.height.unwrap_or(400);
    let color = props.color.clone().unwrap_or_else(|| "#4f46e5".to_string());

    let plot_task = use_async::<_, _, ()>({
        let id = id.clone();
        async move {
            let trace = Bar::new(x_labels, y_values)
                .marker(Marker::new().color("#10b981"));

            let mut plot = Plot::new();
            plot.add_trace(trace);

            let layout = Layout::new()
                .title(Title::with_text(&title))
                .height(height)
                .x_axis(Axis::new().title(Title::with_text(&x_title)))
                .y_axis(Axis::new().title(Title::with_text(&y_title)))
                .paper_background_color("#1a1f36")
                .plot_background_color("#1a1f36")
                .font(Font::new().color("#ffffff").family("Inter"));

            plot.set_layout(layout);

            plotly::bindings::new_plot(&id, &plot).await;
            Ok(())
        }
    });

    use_effect_with((), move |_| {
        plot_task.run();
        || ()
    });

    html! {
        <div id={id} class="plotly-chart"></div>
    }
}

/// Line Chart Component - For time series and trends
#[derive(Properties, PartialEq)]
pub struct LineChartProps {
    pub id: String,
    pub x_data: Vec<String>,
    pub y_data: Vec<f64>,
    pub title: String,
    pub x_title: String,
    pub y_title: String,
    #[prop_or_default]
    pub height: Option<usize>,
    #[prop_or_default]
    pub line_name: Option<String>,
}

#[function_component(LineChart)]
pub fn line_chart(props: &LineChartProps) -> Html {
    let id = props.id.clone();
    let x_data = props.x_data.clone();
    let y_data = props.y_data.clone();
    let title = props.title.clone();
    let x_title = props.x_title.clone();
    let y_title = props.y_title.clone();
    let height = props.height.unwrap_or(400);
    let line_name = props.line_name.clone().unwrap_or_else(|| "Data".to_string());

    let plot_task = use_async::<_, _, ()>({
        let id = id.clone();
        async move {
            let trace = Scatter::new(x_data, y_data)
                .mode(Mode::LinesMarkers)
                .name(&line_name)
                .marker(Marker::new().size(8).color("#4f46e5"))
                .line(Line::new().color("#4f46e5").width(3.0));

            let mut plot = Plot::new();
            plot.add_trace(trace);

            let layout = Layout::new()
                .title(Title::with_text(&title))
                .height(height)
                .x_axis(Axis::new().title(Title::with_text(&x_title)))
                .y_axis(Axis::new().title(Title::with_text(&y_title)))
                .paper_background_color("#1a1f36")
                .plot_background_color("#1a1f36")
                .font(Font::new().color("#ffffff").family("Inter"));

            plot.set_layout(layout);

            plotly::bindings::new_plot(&id, &plot).await;
            Ok(())
        }
    });

    use_effect_with((), move |_| {
        plot_task.run();
        || ()
    });

    html! {
        <div id={id} class="plotly-chart"></div>
    }
}

/// Multi-Line Chart Component - For comparing multiple series
#[derive(Properties, PartialEq, Clone)]
pub struct Series {
    pub name: String,
    pub x_data: Vec<String>,
    pub y_data: Vec<f64>,
    pub color: String,
}

#[derive(Properties, PartialEq)]
pub struct MultiLineChartProps {
    pub id: String,
    pub series: Vec<Series>,
    pub title: String,
    pub x_title: String,
    pub y_title: String,
    #[prop_or_default]
    pub height: Option<usize>,
}

#[function_component(MultiLineChart)]
pub fn multi_line_chart(props: &MultiLineChartProps) -> Html {
    let id = props.id.clone();
    let series = props.series.clone();
    let title = props.title.clone();
    let x_title = props.x_title.clone();
    let y_title = props.y_title.clone();
    let height = props.height.unwrap_or(400);

    let plot_task = use_async::<_, _, ()>({
        let id = id.clone();
        async move {
            let mut plot = Plot::new();

            for serie in series {
                // Use hardcoded colors for now (can be improved later)
                let trace = Scatter::new(serie.x_data, serie.y_data)
                    .mode(Mode::LinesMarkers)
                    .name(&serie.name)
                    .marker(Marker::new().size(8).color("#4f46e5"))
                    .line(Line::new().color("#4f46e5").width(3.0));

                plot.add_trace(trace);
            }

            let layout = Layout::new()
                .title(Title::with_text(&title))
                .height(height)
                .x_axis(Axis::new().title(Title::with_text(&x_title)))
                .y_axis(Axis::new().title(Title::with_text(&y_title)))
                .paper_background_color("#1a1f36")
                .plot_background_color("#1a1f36")
                .font(Font::new().color("#ffffff").family("Inter"));

            plot.set_layout(layout);

            plotly::bindings::new_plot(&id, &plot).await;
            Ok(())
        }
    });

    use_effect_with((), move |_| {
        plot_task.run();
        || ()
    });

    html! {
        <div id={id} class="plotly-chart"></div>
    }
}

/// Grouped Bar Chart Component - For comparing multiple categories
#[derive(Properties, PartialEq, Clone)]
pub struct BarSeries {
    pub name: String,
    pub values: Vec<f64>,
    pub color: String,
}

#[derive(Properties, PartialEq)]
pub struct GroupedBarChartProps {
    pub id: String,
    pub x_labels: Vec<String>,
    pub series: Vec<BarSeries>,
    pub title: String,
    pub x_title: String,
    pub y_title: String,
    #[prop_or_default]
    pub height: Option<usize>,
}

#[function_component(GroupedBarChart)]
pub fn grouped_bar_chart(props: &GroupedBarChartProps) -> Html {
    let id = props.id.clone();
    let x_labels = props.x_labels.clone();
    let series = props.series.clone();
    let title = props.title.clone();
    let x_title = props.x_title.clone();
    let y_title = props.y_title.clone();
    let height = props.height.unwrap_or(400);

    let plot_task = use_async::<_, _, ()>({
        let id = id.clone();
        async move {
            let mut plot = Plot::new();

            for serie in series {
                // Use hardcoded colors for now
                let trace = Bar::new(x_labels.clone(), serie.values)
                    .name(&serie.name)
                    .marker(Marker::new().color("#10b981"));

                plot.add_trace(trace);
            }

            let layout = Layout::new()
                .title(Title::with_text(&title))
                .height(height)
                .x_axis(Axis::new().title(Title::with_text(&x_title)))
                .y_axis(Axis::new().title(Title::with_text(&y_title)))
                .bar_mode(plotly::layout::BarMode::Group)
                .paper_background_color("#1a1f36")
                .plot_background_color("#1a1f36")
                .font(Font::new().color("#ffffff").family("Inter"));

            plot.set_layout(layout);

            plotly::bindings::new_plot(&id, &plot).await;
            Ok(())
        }
    });

    use_effect_with((), move |_| {
        plot_task.run();
        || ()
    });

    html! {
        <div id={id} class="plotly-chart"></div>
    }
}
