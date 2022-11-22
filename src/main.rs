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
fn _get_phi(n: u32) -> u32 {
    let nums = _get_nums(n);
    nums.len() as u32
}
fn _get_nums(n: u32) -> Vec<u32> {
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
fn _get_phi_2(n: u32) -> u32 {
    let primes = _get_primes(n);
    if primes.len() != 2 {
        println!("n is not acceptable");
    }
    (primes[0] - 1) * (primes[1] - 1)
}
fn _get_primes(n: u32) -> Vec<u32> {
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

enum NavMsg {}
struct NavComponent;
impl Component for NavComponent {
    type Message = NavMsg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div class="navbar">
                    <p class="logotext">{"RSA WASM"}</p>
                </div>
            </div>
        }
    }
}
enum MainPageMsg {}
struct MainPageComponent;
impl Component for MainPageComponent {
    type Message = MainPageMsg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        // let link = ctx.link();
        html! {
            <div>
                <NavComponent/>
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
            encrypted_message: "TU BĘDZIE ODPOWIEDŹ".to_string(),
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
        }
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="RSA">
                <p class="title">{"SZYFROWANIE"}</p>
                <div class="row nums">
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
                <p class="result">{self.encrypted_message.clone()}</p>
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
        let d: u32 = ((letter as u128).pow(e) % n as u128) as u32;
        nums.push(d);
    }
    nums
}
