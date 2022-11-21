use yew::prelude::*;
// fn main() {
//     let args: Vec<String> = std::env::args().skip(1).collect();
//     if args.len() != 1 {
//         println!("not enought arguments");
//     }
//     let n: u32 = args[0].parse::<u32>().unwrap();
//     let e: u32 = args[1].parse::<u32>().unwrap();

//     let phi = get_phi(n);
//     let d: u32 = (1 % phi) / e;

//     println!("phi for number {} = {}", args[0], phi);
//     println!("(n,e,d) = ({},{},{})", n, e, d);

//     println!("primes of {}: {:?}", n, get_primes(n));
//     println!("phi: {}", get_phi_2(n));
// }
// Pierwsza metoda na obliczenie phi
fn get_phi(n: u32) -> u32 {
    let nums = get_nums(n);
    nums.len() as u32
}
fn get_nums(n: u32) -> Vec<u32> {
    let mut nums: Vec<u32> = vec![1];
    for i in 2..n {
        let mut has_common_divider = false;
        for j in 2..n {
            if i % j == 0 && n % j == 0 {
                // println!("{} ma dzielnik", i);
                has_common_divider = true;
                break;
            }
        }
        if has_common_divider == false {
            nums.push(i);
        }
    }

    nums
}
// Druga metoda na obliczenie phi
fn get_phi_2(n: u32) -> u32 {
    let primes = get_primes(n);
    if primes.len() != 2 {
        println!("n is not acceptable");
    }
    (primes[0] - 1) * (primes[1] - 1)
}
fn get_primes(n: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::new();
    let mut x = n;
    while x != 1 {
        for i in 2..=x {
            if x % i == 0 {
                x = x / i;
                primes.push(i);
                break;
            }
        }
    }
    primes
}

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
            <div>
                <div class="navbar">
                    <p class="logotext">{"RSA WASM"}</p>
                    <img src="https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fi.redd.it%2Fntpsyag1d3p61.jpg&f=1&nofb=1&ipt=8453c0e8d59a49cb045a525913b2904cebb0ea3177eb7ca5d5e503cd03619b0d&ipo=images" alt="karpportal logo"/>
                </div>
            </div>
        }
    }
}
enum MainPageMsg {
    Good,
}
struct MainPageComponent;
impl Component for MainPageComponent {
    type Message = MainPageMsg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
                <NavComponent/>
                // <CounterComponent/>
                <RsaComponent/>
            </div>
        }
    }
}
fn main() {
    yew::start_app::<MainPageComponent>();
}
enum RsaMsg {
    N(u32),
    E(u32),
    UpdateN(u32),
    UpdateE(u32),
}
struct RsaComponent {
    n: u32,
    e: u32,
    phi: u32,
    l: u32,
}
impl Component for RsaComponent {
    type Message = RsaMsg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            phi: 0,
            n: 0,
            e: 0,
            l: 1,
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            RsaMsg::UpdateN(content) => self.n = content,
            RsaMsg::UpdateE(content) => self.e = content,
            _ => (),
        }
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="RSA">
                <div class="row">
                    <div class="indicator">
                        <p>{"N: "}</p>
                        <p>{self.n}</p>
                    </div>
                    <div class="indicator">
                        <p>{"E: "}</p>
                        <p>{self.e}</p>
                    </div>
                </div>
                <div class = "row">
                    <input type="number" oninput={link.callback(|event:InputEvent| RsaMsg::UpdateN(event.data().unwrap().parse::<u32>().unwrap()))}/>
                    <input type="number" oninput={link.callback(|event:InputEvent| RsaMsg::UpdateE(event.data().unwrap().parse::<u32>().unwrap()))}/>
                </div>
            </div>
        }
    }
}
