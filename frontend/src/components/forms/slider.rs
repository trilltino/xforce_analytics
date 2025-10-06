use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SliderProps {
    pub label: String,
    pub value: f64,
    pub onchange: Callback<f64>,
    pub min: f64,
    pub max: f64,

    #[prop_or(1.0)]
    pub step: f64,

    #[prop_or_default]
    pub help_text: Option<String>,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or_default]
    pub format_value: Option<Callback<f64, String>>,
}

#[function_component(Slider)]
pub fn slider(props: &SliderProps) -> Html {
    let oninput = {
        let callback = props.onchange.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(value) = input.value().parse::<f64>() {
                callback.emit(value);
            }
        })
    };

    let display_value = if let Some(formatter) = &props.format_value {
        formatter.emit(props.value)
    } else {
        format!("{:.0}", props.value)
    };

    html! {
        <div class="form-group slider-group">
            <div class="slider-header">
                <label class="form-label">{&props.label}</label>
                <span class="slider-value">{display_value}</span>
            </div>

            <input
                type="range"
                class="form-slider"
                value={props.value.to_string()}
                min={props.min.to_string()}
                max={props.max.to_string()}
                step={props.step.to_string()}
                {oninput}
                disabled={props.disabled}
            />

            if let Some(help) = &props.help_text {
                <div class="field-hint">{help}</div>
            }
        </div>
    }
}
