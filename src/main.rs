use gloo_console::log;
use wasm_bindgen::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;

// const offset: u8 = 60;

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
                <DecryptComponent/>
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
    UpdateOffset(u8),
    Encrypt,
}
struct RsaComponent {
    n: u32,
    e: u32,
    message: String,
    nums: Vec<u32>,
    encrypted_message: String,
    hint: String,
    hint2: String,
    offset: u8,
}
impl Component for RsaComponent {
    type Message = RsaMsg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            n: 0,
            e: 0,
            message: String::new(),
            encrypted_message: "TU BĘDZIE ODPOWIEDŹ".to_string(),
            nums: Vec::new(),
            hint: String::new(),
            hint2: String::new(),
            offset: 60,
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
            RsaMsg::UpdateOffset(content) => self.offset = content,
            RsaMsg::UpdateE(content) => self.e = content,
            RsaMsg::UpdateMessage(content) => self.message = content,
            RsaMsg::Encrypt => {
                let result = encrypt(self.message.clone(), self.n, self.e, self.offset);
                self.nums = result.0;
                if result.1 {
                    self.hint = "WARNING: stack overflow on calculating to the power".to_string();
                } else {
                    self.hint = String::new();
                }
                if result.2 {
                    self.hint2 = "WARNING: offset to large failed to subtract".to_string();
                } else {
                    self.hint2 = String::new();
                }
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
                <p class="hint">{self.hint2.clone()}</p>
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
                    <div class="indicator">
                        <p>{"Offset:"}</p>
                        <p>{self.offset}</p>
                    </div>
                </div>

                <div class = "row">
                    <p>{"N"}</p>
                    <input type="number" class="number-input" placeholder="enter N" onchange={link.callback(|event:Event| RsaMsg::UpdateN(event.target().unwrap().unchecked_into::<HtmlInputElement>().value().parse::<u32>().unwrap()))}/>
                    <p>{"E"}</p>
                    <input type="number" class="number-input" placeholder="enter E" onchange={link.callback(|event:Event| RsaMsg::UpdateE(event.target().unwrap().unchecked_into::<HtmlInputElement>().value().parse::<u32>().unwrap()))}/>
                    <p>{"Offset"}</p>
                    <input type="number" class="number-input" placeholder="offset" onchange={link.callback(|event:Event| RsaMsg::UpdateOffset(event.target().unwrap().unchecked_into::<HtmlInputElement>().value().parse::<u8>().unwrap()))}/>
                </div>
                    <input type="text" class="text-input" placeholder="enter secret message" onchange={link.callback(|event:Event| RsaMsg::UpdateMessage(event.target().unwrap().unchecked_into::<HtmlInputElement>().value()))}/>
                <button onclick={link.callback(|_|RsaMsg::Encrypt)}>{"ENCRYPT"}</button>
                <p class="result">{self.encrypted_message.clone()}</p>
            </div>
        }
    }
}

struct DecryptComponent {
    n: u32,
    e: u32,
    d: u32,
    nums: Vec<u32>,
    decrypted_message: String,
    hint: String,
    hint2: String,
    offset: u8,
}
enum DecryptMsg {
    UpdateN(u32),
    UpdateE(u32),
    UpdateDigits(String),
    UpdateOffset(u8),
    Decrypt,
}
impl Component for DecryptComponent {
    type Message = DecryptMsg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            d: 0,
            n: 0,
            e: 0,
            decrypted_message: "TU BĘDZIE ODPOWIEDŹ".to_string(),
            nums: Vec::new(),
            hint: String::new(),
            hint2: String::new(),
            offset: 60,
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DecryptMsg::UpdateN(content) => {
                self.n = content;
                if get_primes(content).len() == 2 {
                    self.e = find_e(content);
                    self.d = get_d(self.n, self.e);
                    self.hint = String::new();
                } else {
                    self.hint =
                        "WARNING: N is not made out of two prime numbers, E cannot be found"
                            .to_string();
                }
            }
            DecryptMsg::UpdateE(content) => {
                self.e = content;
                self.d = get_d(self.n, self.e);
            }
            DecryptMsg::UpdateOffset(content) => self.offset = content,
            DecryptMsg::UpdateDigits(content) => {
                self.hint2 = "".to_string();
                self.nums = content
                    .split(" ")
                    .map(|f| match f.to_string().parse::<u32>() {
                        Ok(v) => v,
                        Err(e) => {
                            self.hint2 = e.to_string();
                            0
                        }
                    })
                    .collect()
            }
            DecryptMsg::Decrypt => {
                self.d = get_d(self.n, self.e);
                let result = decrypt(&self.nums, self.n, self.d, self.offset);
                self.decrypted_message = result.0.clone();
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
                <p class="title">{"ODSZYFROWANIE"}</p>
                <p class="hint">{self.hint.clone()}</p>
                <p class="hint">{self.hint2.clone()}</p>
                <div class="row nums">
                    <div class="indicator">
                        <p>{"N:"}</p>
                        <p>{self.n}</p>
                    </div>
                    <div class="indicator">
                        <p>{"E:"}</p>
                        <p>{self.e}</p>
                    </div>
                    <div class="indicator">
                        <p>{"Offset:"}</p>
                        <p>{self.offset}</p>
                    </div>
                    <div class="indicator">
                        <p>{"D:"}</p>
                        <p>{self.d}</p>
                    </div>
                </div>

                <div class = "row">
                    <p>{"N"}</p>
                    <input type="number" class="number-input" placeholder="enter N" onchange={link.callback(|event:Event| DecryptMsg::UpdateN(event.target().unwrap().unchecked_into::<HtmlInputElement>().value().parse::<u32>().unwrap()))}/>
                    <p>{"E"}</p>
                    <input type="number" class="number-input" placeholder="enter E" onchange={link.callback(|event:Event| DecryptMsg::UpdateE(event.target().unwrap().unchecked_into::<HtmlInputElement>().value().parse::<u32>().unwrap()))}/>
                    <p>{"Offset"}</p>
                    <input type="number" class="number-input" placeholder="offset" onchange={link.callback(|event:Event| DecryptMsg::UpdateOffset(event.target().unwrap().unchecked_into::<HtmlInputElement>().value().parse::<u8>().unwrap()))}/>
                </div>
                    <input type="text" class="text-input" placeholder="enter secret code" onchange={link.callback(|event:Event| DecryptMsg::UpdateDigits(event.target().unwrap().unchecked_into::<HtmlInputElement>().value()))}/>
                <button onclick={link.callback(|_|DecryptMsg::Decrypt)}>{"DECRYPT"}</button>
                <p class="result">{self.decrypted_message.clone()}</p>
            </div>
        }
    }
}

fn refactor(nums: &[u32]) -> String {
    let mut message = String::new();
    for i in 0..nums.len() {
        message += nums[i].to_string().as_str();
        if i != nums.len() - 1 {
            message += " ";
        }
    }
    message
}
fn encrypt(text: String, n: u32, e: u32, offset: u8) -> (Vec<u32>, bool, bool) {
    let mut nums: Vec<u32> = Vec::new();
    let mut powered: (u128, bool) = (1, false);
    let mut subed: (u128, bool) = (1, false);
    for letter in text.chars() {
        log!(letter as u32);
        subed = (letter as u128).overflowing_sub(offset as u128);
        powered = subed.0.overflowing_pow(e);
        let c: u32 = (powered.0 % n as u128) as u32;
        nums.push(c);
    }
    // log!((('A' as u32) as u8 as char).to_string());
    (nums, powered.1, subed.1)
}
fn decrypt(nums: &[u32], n: u32, d: u32, offset: u8) -> (String, bool) {
    let mut out = String::new();
    log!("d is equal to");
    log!(d);
    let mut powered: (u128, bool) = (1, false);
    for i in 0..nums.len() {
        // log!(nums[i] as u128);
        powered = (nums[i] as u128).overflowing_pow(d);
        // log!(p);
        let m = powered.0 % n as u128;
        // log!(num.clone());
        log!("gut");
        log!(m as u32);
        out += &((m as u8 + offset) as char).to_string();
    }
    (out, powered.1)
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
