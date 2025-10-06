use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct InputProps {
    pub label: String,
    pub value: String,
    pub onchange: Callback<String>,

    #[prop_or("text".to_string())]
    pub input_type: String,

    #[prop_or(false)]
    pub required: bool,

    #[prop_or_default]
    pub placeholder: Option<String>,

    #[prop_or_default]
    pub help_text: Option<String>,

    #[prop_or_default]
    pub error: Option<String>,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or_default]
    pub min: Option<String>,

    #[prop_or_default]
    pub max: Option<String>,

    #[prop_or_default]
    pub step: Option<String>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let oninput = {
        let callback = props.onchange.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            callback.emit(input.value());
        })
    };

    let mut classes = classes!("form-input");
    if props.error.is_some() {
        classes.push("input-error");
    }

    html! {
        <div class="form-group">
            <label class="form-label">
                {&props.label}
                if props.required {
                    <span class="required">{" *"}</span>
                }
            </label>

            <input
                type={props.input_type.clone()}
                class={classes}
                value={props.value.clone()}
                {oninput}
                placeholder={props.placeholder.clone().unwrap_or_default()}
                required={props.required}
                disabled={props.disabled}
                min={props.min.clone()}
                max={props.max.clone()}
                step={props.step.clone()}
            />

            if let Some(error) = &props.error {
                <div class="field-error">{error}</div>
            } else if let Some(help) = &props.help_text {
                <div class="field-hint">{help}</div>
            }
        </div>
    }
}
