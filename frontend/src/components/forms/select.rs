use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SelectProps {
    pub label: String,
    pub value: String,
    pub onchange: Callback<String>,
    pub options: Vec<(String, String)>, // (value, label)

    #[prop_or(false)]
    pub required: bool,

    #[prop_or_default]
    pub help_text: Option<String>,

    #[prop_or_default]
    pub error: Option<String>,

    #[prop_or(false)]
    pub disabled: bool,
}

#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
    let onchange = {
        let callback = props.onchange.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            callback.emit(select.value());
        })
    };

    let mut classes = classes!("form-select");
    if props.error.is_some() {
        classes.push("select-error");
    }

    html! {
        <div class="form-group">
            <label class="form-label">
                {&props.label}
                if props.required {
                    <span class="required">{" *"}</span>
                }
            </label>

            <select
                class={classes}
                value={props.value.clone()}
                {onchange}
                required={props.required}
                disabled={props.disabled}
            >
                {for props.options.iter().map(|(value, text)| {
                    html! {
                        <option value={value.clone()} selected={&props.value == value}>
                            {text}
                        </option>
                    }
                })}
            </select>

            if let Some(error) = &props.error {
                <div class="field-error">{error}</div>
            } else if let Some(help) = &props.help_text {
                <div class="field-hint">{help}</div>
            }
        </div>
    }
}
