use wasm_bindgen::*;
use web_sys::HtmlInputElement;
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
    Nothin,
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
    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        // let link = ctx.link();
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
    UpdateN(u32),
    UpdateE(u32),
    UpdateMessage(String),
    Encrypt,
}
struct RsaComponent {
    n: u32,
    e: u32,
    message: String,
    nums: Vec<u32>,
    encrypted_message: String,
    // phi: u32,
    // l: u32,
}
impl Component for RsaComponent {
    type Message = RsaMsg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            // phi: 0,
            n: 0,
            e: 0,
            message: String::new(),
            encrypted_message: String::new(),
            nums: Vec::new(),
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            RsaMsg::UpdateN(content) => self.n = content,
            RsaMsg::UpdateE(content) => self.e = content,
            RsaMsg::UpdateMessage(content) => self.message = content,
            RsaMsg::Encrypt => {
                self.nums = encrypt(self.message.clone(), self.n, self.e);
                self.encrypted_message = refactor(&self.nums);
            }
            _ => (),
        }
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="RSA">
                <p class="title">{"SZYFROWANIE"}</p>
                <div class="row">
                    <div class="indicator">
                        <p>{"N:"}</p>
                        <p>{self.n}</p>
                    </div>
                    <div class="indicator">
                        <p>{"E:"}</p>
                        <p>{self.e}</p>
                    </div>
                </div>

                <div class = "row">
                    <p>{"N"}</p>
                    <input type="number" class="number-input" placeholder="enter N" onchange={link.callback(|event:Event| RsaMsg::UpdateN(event.target().unwrap().unchecked_into::<HtmlInputElement>().value().parse::<u32>().unwrap()))}/>
                    <p>{"E"}</p>
                    <input type="number" class="number-input" placeholder="enter E" onchange={link.callback(|event:Event| RsaMsg::UpdateE(event.target().unwrap().unchecked_into::<HtmlInputElement>().value().parse::<u32>().unwrap()))}/>
                </div>
                    <input type="text" class="text-input" placeholder="enter secret message" onchange={link.callback(|event:Event| RsaMsg::UpdateMessage(event.target().unwrap().unchecked_into::<HtmlInputElement>().value()))}/>
                <button onclick={link.callback(|_|RsaMsg::Encrypt)}>{"ENCRYPT"}</button>
                <p>{self.encrypted_message.clone()}</p>
            </div>
        }
    }
}

fn refactor(nums: &[u32]) -> String {
    let mut message = String::new();
    for num in nums {
        message += num.to_string().as_str();
        message += " ";
    }
    message
}
fn encrypt(text: String, n: u32, e: u32) -> Vec<u32> {
    let mut nums: Vec<u32> = Vec::new();
    for letter in text.chars() {
        let d = ((letter as u32).pow(e)) % n;
        nums.push(d);
    }
    nums
}
