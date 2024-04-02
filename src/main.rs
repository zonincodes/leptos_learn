use std::marker::PhantomData;

use leptos::*;
fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (x, set_x) = create_signal(0);
    let double_count = move || x() * 2;

    view! {
        <div>
            <button
            on:click = move |_| { set_x.update(|n| *n += 10); }
            style="position: absolute"
            // and toggle individual CSS properties with 'style'
            style:left = move|| format!("{}px", x() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", x(), 100)
            style:max-width="400px"
            style=("--columns", x)
            >
            "Click me: " <span
                style:color="white"
            >{x}</span>
            </button>
            <br/>
            <ProgressBar max = 100 progress=x/>{x}
            <br/>
            <ProgressBar max = 200 progress=double_count/>{double_count}
            <br/>
            <SizeOf<Demo>/>
            <br/>
            <SizeOf<String>/>
            <br/>
            <Complex />
            <br/>
            <ControlledInput/>
            <br/>
            <NumericInput />
            <br/>
            <NumericInputErrorBoundary />
        </div>
    }
}

#[component]
fn SizeOf<T>(#[prop(optional)] _ty: PhantomData<T>) -> impl IntoView
where
    T: Sized,
{
    std::mem::size_of::<T>()
}

struct Demo<'a> {
    name: &'a str,
    email: &'a str,
}

#[component]
fn ProgressBar<F>(
    // mark this prop optional
    // you can specify it or not when you use <ProgressBar/>
    #[prop(default = 100)] max: u16,
    progress: F,
) -> impl IntoView
where
    F: Fn() -> i32 + 'static,
{
    view! {
        <progress
        max=max
        value= progress>

        </progress>
    }
}

#[component]
fn ProgressBar2(
    #[prop(default = 100)] max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress max = max value =progress />
    }
}

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: RwSignal<i32>,
}

#[component]
fn Complex() -> impl IntoView {
    let (data, set_data) = create_signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: create_rw_signal(10),
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: create_rw_signal(20),
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: create_rw_signal(15),
        },
    ]);

    view! {
        // when we click, we update each row,
        // doubling its value
        <button on:click=move |_| {
            set_data.update(|data|{
                for row in data {
                    row.value.update(|value| *value *= 2);
                }
            });
            logging::log!("{:?}", data.get());
        }
        > "Update values"</button>
        <For
        each=data
        key=|state| state.key.clone()
        let:child>
        <p>{move|| child.value} </p>
        </For>
    }
}


#[component]
fn ControlledInput() -> impl IntoView{

    let (name, set_name) = create_signal("".to_string());

    view! {
        <input type="text"
        placeholder="Enter Name"
            on:input= move |ev| {
                set_name(event_target_value(&ev))
            }
            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}

// Error Handling 
#[component]
fn NumericInput () -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    // when input changes, try ti parse a number from the input
    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());
    view! {
        <label>
        "Type a number (or not!)"
        <input type="number" on:input=on_input/>
        <p>"You Entered "</p>
        <strong>{value}</strong>
        </label>
    }
}

// Error boundary
#[component]
fn NumericInputErrorBoundary() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));
    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {
        <h1> "Error Handling"</h1>
        <label>
            <p>"Type a number (or something that is not a number!)"</p>
            <input  on:input=on_input/>
            <ErrorBoundary
            // the fallback receives a signal containing current errors
            fallback=|errors| view! {
                <div calss="errors">
                <p>"Not a number"! Errors: </p>
                // we van render a list of errorss as strings, if we'd like
            <ul>
            {
                move || errors.get()
                .into_iter()
                .map(|(_, e)| view! { <li>{e.to_string()} </li>})
                .collect_view()
            }
            </ul>
            </div>
            }
            >
            <p>"Value is: " {move || value.get()}</p>
            </ErrorBoundary>
        </label>
    }

}