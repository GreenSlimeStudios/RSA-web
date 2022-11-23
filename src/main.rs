use gloo_console::log;
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
    Decrypt,
}
struct RsaComponent {
    n: u32,
    e: u32,
    message: String,
    nums: Vec<u32>,
    encrypted_message: String,
    hint: String,
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
            hint: String::new(),
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            RsaMsg::UpdateN(content) => {
                self.n = content;
                if get_primes(content).len() == 2 {
                    self.e = find_e(content);
                    self.hint = String::new();
                } else {
                    self.hint =
                        "N is not made out of two prime numbers, E cannot be found".to_string();
                }
            }
            RsaMsg::UpdateE(content) => self.e = content,
            RsaMsg::UpdateMessage(content) => self.message = content,
            RsaMsg::Encrypt => {
                let result = encrypt(self.message.clone(), self.n, self.e);
                self.nums = result.0;
                if result.1 {
                    self.hint = "WARNING: stack overflow on calculating to the power".to_string();
                }
                self.encrypted_message = refactor(&self.nums);
            }
            RsaMsg::Decrypt => {
                let result = decrypt(&self.nums, self.n, self.e);
                log!(result.0);
                if result.1 {
                    self.hint = "WARNING: stack overflow on calculating to the power".to_string();
                }
            }
        }
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="RSA">
                <p class="title">{"SZYFROWANIE"}</p>
                <p class="hint">{self.hint.clone()}</p>
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
                <button onclick={link.callback(|_|RsaMsg::Decrypt)}>{"DECRYPT"}</button>
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
fn encrypt(text: String, n: u32, e: u32) -> (Vec<u32>, bool) {
    let mut nums: Vec<u32> = Vec::new();
    let mut p: (u128, bool) = (1, false);
    for letter in text.chars() {
        log!(letter as u32);
        p = (letter as u128).overflowing_pow(e);
        let c: u32 = (p.0 % n as u128) as u32;
        nums.push(c);
    }
    // log!((('A' as u32) as u8 as char).to_string());
    (nums, p.1)
}
fn decrypt(nums: &[u32], n: u32, e: u32) -> (String, bool) {
    let mut out = String::new();
    let d = get_d(n, e);
    log!("d is equal to");
    log!(d);
    let mut p: (u128, bool) = (1, false);
    for i in 0..nums.len() {
        // log!(nums[i] as u128);
        p = (nums[i] as u128).overflowing_pow(d);
        // log!(p);
        let m = p.0 % n as u128;
        // log!(num.clone());
        log!("gut");
        log!(m as u32);
        out += &((m as u8) as char).to_string();
    }
    (out, p.1)
}
fn get_d(n: u32, e: u32) -> u32 {
    let mut out: u32 = 1;
    for d in 0..=n {
        if (d * e) % get_phi_2(n) == 1 {
            out = d;
        }
    }
    out
}
fn find_e(n: u32) -> u32 {
    let mut e: u32 = 2;
    let phi = get_phi_2(n);
    let coprime_n = get_nums(n);
    let coprime_phi = get_nums(phi);

    for i in 2..phi {
        if coprime_n.contains(&i) && coprime_phi.contains(&i) {
            e = i;
        }
    }
    e
}
