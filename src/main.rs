mod raw;

use gloo_net::http::Request;
use pulldown_cmark::html::push_html;
use pulldown_cmark::Options;
use pulldown_cmark::Parser;
use raw::SafeHtml;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/:path")]
    SubPage { path: String },
}

enum Msg {
    Fetch,
    Return(String),
}

struct Model {
    data: String,
}

#[derive(PartialEq, Properties)]
struct ModelProperties {
    name: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ModelProperties;

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.data.is_empty() {
            ctx.link().send_message(Msg::Fetch);
        }

        let options = Options::empty();
        let parser = Parser::new_ext(&self.data, options);

        let mut out = String::new();

        push_html(&mut out, parser);

        html! {
            <SafeHtml html={out}/>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Fetch => {
                let name = ctx.props().name.clone();

                ctx.link().send_future(async move {
                    let url = format!(
                        "https://raw.githubusercontent.com/purlinux/fyi/main/{}.md",
                        name
                    );

                    let resp = Request::get(&url).send().await.unwrap();
                    let text = resp.text();

                    Msg::Return(text.await.unwrap())
                });

                false
            }
            Msg::Return(val) => {
                self.data = val;
                true
            }
        }
    }

    fn create(_: &Context<Self>) -> Self {
        Self {
            data: String::from(""),
        }
    }
}

fn switch(route: &Route) -> Html {
    match route {
        Route::SubPage { path } => {
            html! {
                <Model name={path.clone()}/>
            }
        }
        _ => html! {
            <Model name={"home"}/>
        },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<Main>();
}
