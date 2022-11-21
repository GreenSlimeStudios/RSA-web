use yew::prelude::*;

struct CounterComponent {
    count: i64,
}
enum Msg {
    AddOne,
}
impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.count += 1;
                true
            } // _ => false,
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    // fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    // fn destroy(&mut self, ctx: &Context<Self>) {}

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="container">
                <p> {self.count} </p>
                <button onclick={link.callback(|_|Msg::AddOne)}>{"+1"}</button>
                <p> {self.count} </p>
            </div>
        }
    }
}
enum NavMsg {
    Sussy,
}
struct NavComponent;
impl Component for NavComponent {
    type Message = NavMsg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="navbar">
                <p class="logotext">{"Karpportal"}</p>
                <img src="https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fi.redd.it%2Fntpsyag1d3p61.jpg&f=1&nofb=1&ipt=8453c0e8d59a49cb045a525913b2904cebb0ea3177eb7ca5d5e503cd03619b0d&ipo=images" alt="karpportal logo"/>
            </div>
        }
    }
}
fn main() {
    yew::start_app::<NavComponent>();
}
