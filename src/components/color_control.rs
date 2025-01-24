pub mod colorization;

use colorization::Colorization;
use palette::Srgb;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::{
    button::{ButtonAction, ButtonStyle, IconButton},
    IconType,
};

#[derive(Clone, PartialEq, Properties)]
pub struct MultiColorControlProps {
    pub id: String,
    pub default: Colorization,
    pub state_handle: UseStateHandle<Colorization>,
}

#[function_component(MultiColorControl)]
pub fn multi_color_control(props: &MultiColorControlProps) -> Html {
    let mut node_controls: Vec<Html> = vec![];

    for (index, color) in (*props.state_handle).clone().0.iter().enumerate() {
        let control = html! {
            <ColorNodeControl index={index} id={props.id.clone()} color={*color} state_handle={props.state_handle.clone()}/>
        };

        node_controls.push(control);
    }

    html! {
        <ContextProvider<UseStateHandle<Colorization>> context={ props.state_handle.clone() }>
            <div class={classes!("color-control", "setting")} id={ "color_control" }>
                { node_controls }
                <AddColorControl/>
            </div>
        </ContextProvider<UseStateHandle<Colorization>>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ColorNodeControlProps {
    pub index: usize,
    pub id: String,
    pub color: Srgb<f32>,
    pub state_handle: UseStateHandle<Colorization>,
}

#[function_component(ColorNodeControl)]
pub fn color_node_control(props: &ColorNodeControlProps) -> Html {
    let input_node_ref = use_node_ref();

    let displayed_color: String = {
        let x: Srgb<u8> = props.color.into_format();
        format!("#{x:x}")
    };

    let oninput = {
        let input_node_ref = input_node_ref.clone();

        let state = props.state_handle.clone();
        let index = props.index;

        Callback::from(move |_| {
            let input = input_node_ref.cast::<HtmlInputElement>();

            if let Some(x) = input {
                let color = x
                    .value()
                    .parse()
                    .unwrap_or(Srgb::new(255, 0, 255))
                    .into_format();

                let mut colorization = (*state).clone();

                colorization.0[index] = color;

                state.set(colorization);
            }
        })
    };

    let id = format!("{}-{}", props.id.clone(), props.index);

    html! {
        <div class={classes!("color-node")}>
            <span style="display:flex; flex-direction:horizontal;">
                <label for={ id.clone() }>{ &displayed_color.clone() }</label>
                <RemoveColorControl index={ props.index }/>
            </span>
            <input ref={ input_node_ref } { oninput } type={ "color" } id={ id.clone() } name={ id.clone() } value={ displayed_color.clone() }/>
        </div>
    }
}

#[function_component(AddColorControl)]
pub fn add_color_control() -> Html {
    let state = use_context::<UseStateHandle<Colorization>>()
        .expect("There was no Colorization context for this ColorControl!");

    let onclick = {
        let state = state.clone();

        Callback::from(move |_| {
            let mut colorization = (*state).clone();

            colorization.0.push(palette::named::WHITE.into_format());

            state.set(colorization);
        })
    };

    html! {
        <IconButton name={ "add color" } action={ ButtonAction::StateChange(onclick) } icon={ IconType::NerdFont("nf-md-plus".to_string()) } style={ ButtonStyle::Screen } classes={ classes!("x-small") } />
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct RemoveColorControlProps {
    pub index: usize,
}

#[function_component(RemoveColorControl)]
pub fn remove_color_control(props: &RemoveColorControlProps) -> Html {
    let state = use_context::<UseStateHandle<Colorization>>()
        .expect("There was no Colorization context for this ColorControl!");

    let onclick = {
        let state = state.clone();
        let index = props.index;

        Callback::from(move |_| {
            let mut colorization = (*state).clone();

            colorization.0.remove(index);

            state.set(colorization);
        })
    };

    html! {
        <IconButton name={ "remove color" } action={ ButtonAction::StateChange(onclick) } icon={ IconType::NerdFont("nf-md-minus_box_outline".to_string()) } style={ ButtonStyle::Bare } classes={ classes!("xx-small") } />
    }
}
