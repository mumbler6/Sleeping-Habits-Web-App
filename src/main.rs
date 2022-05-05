use yew::prelude::*;
use gloo::timers::callback::{Interval};
use substring::Substring;
use web_sys::*;

enum Msg {
    UpdateTime,
    InputValue(String),
    Delete,
    CheckTime,
    PlaySound,
    StopSound,
}

struct CounterComponent {
    sleep_label: String,
    sleep_time: String,
    interval: Interval,
    check_interval: Interval,
    current_time: String,
    alarm_audio: HtmlAudioElement,
}

fn get_current_time() -> String {
    let date = js_sys::Date::new_0();
    String::from(date.to_locale_time_string("en-US"))
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let clock_handle = {
            let link = _ctx.link().clone();
            Interval::new(1, move || link.send_message(Msg::UpdateTime))
        };
        let check_handle = {
            let link = _ctx.link().clone();
            Interval::new(1, move || link.send_message(Msg::CheckTime))
        };

        let sl = "I want to sleep at... ";
        let st = "h:mm:ss AM/PM";

        let main_alarm = web_sys::HtmlAudioElement::new_with_src("https://wiki.teamfortress.com/w/images/6/6c/Heavy_specialcompleted10.wav");

        Self {
            sleep_time: st.to_string(),
            sleep_label: sl.to_string(),
            interval: clock_handle,
            check_interval: check_handle,
            current_time: get_current_time(),
            alarm_audio: main_alarm.unwrap(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateTime => {
                self.current_time = get_current_time();
                true
            }
            Msg::InputValue(value) => {
                self.sleep_time += &value;
                true
            }
            Msg::Delete => {
                self.sleep_time = "".to_string();
                true
            }   
            Msg::CheckTime => {
                if self.sleep_time == self.current_time {
                    _ctx.link().clone().send_message(Msg::PlaySound);
                }
                true
            }
            Msg::PlaySound => {
                self.alarm_audio.set_loop(true);
                self.alarm_audio.play().unwrap();
                true
            }
            Msg::StopSound => {
                self.alarm_audio.set_loop(false);
                true
            }
        }

    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
            <div class="container">
                <p> { self.current_time.clone() }</p>
                <a> { self.sleep_label.clone() } </a>
                <input type="text" value={self.sleep_time.clone()} oninput={link.callback(|e: InputEvent| Msg::InputValue(e.data().unwrap()))} />
                <button onclick={link.callback(|_| Msg::Delete)}>{ '\u{232B}' }</button>
            </div>
            <div class="button">
                <button onclick={link.callback(|_| Msg::StopSound)}>{ "Stop Alarm" }</button>
            </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<CounterComponent>();
}