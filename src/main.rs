use yew::prelude::*;

enum Msg {
    Plus,
    Minus,
}

struct CounterComponent{
    count:i32,
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {count:0}
    }
    
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Plus => {
                self.count += 1;
                true //for component rerender
            },
            Msg::Minus => {
                self.count -= 1;
                true
            }
        }
    }

    fn view(&self, ctx:&Context<Self>) -> Html{
        let link = ctx.link();
        html!{
            <div class="counter">
                <h1>{self.count}</h1>
                <button class="plus" onclick={link.callback(|_| Msg::Plus)}>
                {"+"}
                </button>
                <button class="minus" onclick={link.callback(|_| Msg::Minus)}>
                {"-"}
                </button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<CounterComponent>();
}
