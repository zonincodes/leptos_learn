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
            <ProgressBar max = 100 progress=double_count/>{double_count}
            
        </div>
    }
}


#[component]
fn ProgressBar<F: Fn() -> i32 + 'static>(
    // mark this prop optional
    // you can specify it or not when you use <ProgressBar/> 
    #[prop(optional)]
    max: u16,
    progress: F
) -> impl IntoView {
    view! {
        <progress
        max=max
        value= progress>
        
        </progress>
    }
}