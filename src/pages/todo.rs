use crate::components::counter_btn::Button;
use leptos::*;

#[derive(Debug, Clone)]
struct Item {
    id: i32,
    value: String,
}

/// Default Home Page
#[component]
pub fn Todo() -> impl IntoView {
    let (name, set_name) = create_signal("".to_string());
    let (items, set_items) = create_signal(vec![
            Item {
            id: 1,
            value: "Test".to_string()
        },
    ]);
    let last_item_id = move || items().iter().map(|item| item.id).max();

    let input_element: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let mut new_items = items();
        let new_id = last_item_id().unwrap_or_default() + 1;
        new_items.push(Item {
            id: new_id,
            value: input_element()
                    .expect("<input> should be mounted")
                    .value()
        });
        set_items.set(new_items);
    };

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="container">
                <h1>"Todo list"</h1>                
                <form on:submit=on_submit class="container"> // on_submit defined below
                    <input
                        type="text"
                        value=name
                        node_ref=input_element
                    />
                    <input type="submit" value="Submit"/>
                </form>
                <ul>
                    <For
                        each=items
                        key=|state| state.id.clone()
                        let:child
                    >
                        <li class="container">
                            <div>
                                <p>{child.value}</p>
                                <button on:click=move |_| {
                                    set_items.update(|items| {
                                        items.retain(|item| &item.id != &child.id)
                                    });
                                }>remover</button>
                            </div>
                        </li>
                    </For>
                </ul>
            </div>
        </ErrorBoundary>
    }
}
